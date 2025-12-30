// Generated macro for derive_lookup_table_address (function)
macro_rules! Depcrate_instructionderive_lookup_table_address {
() => {
// Module: crate::instruction
// Provides: {"derive_lookup_table_address"}
// Dependencies: {}
# [doc = " Derives the address of an address table account from a wallet address and a recent block's slot."] pub fn derive_lookup_table_address (authority_address : & Pubkey , recent_block_slot : Slot ,) -> (Pubkey , u8) { Pubkey :: find_program_address (& [authority_address . as_ref () , & recent_block_slot . to_le_bytes ()] , & id () ,) }
};
}
