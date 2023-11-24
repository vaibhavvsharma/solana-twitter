use anchor_lang::prelude::*;

declare_id!("DTpzL966JaPFA1VfoyF5CNdYkSoKPLDXqJErpuJvjcTK");

#[program]
pub mod solana_twitter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
