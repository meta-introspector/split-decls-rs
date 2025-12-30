// Generated macro for create_lookup_table (function)
macro_rules! Depcrate_instructioncreate_lookup_table {
() => {
// Module: crate::instruction
// Provides: {"create_lookup_table"}
// Dependencies: {}
# [doc = " Constructs an instruction to create a table account and returns"] # [doc = " the instruction and the table account's derived address."] # [doc = ""] # [doc = " # Note"] # [doc = ""] # [doc = " This instruction doesn't require the authority to be a signer but"] # [doc = " until v1.12 the address lookup table program still requires the"] # [doc = " authority to sign the transaction."] # [cfg (feature = "bincode")] pub fn create_lookup_table (authority_address : Pubkey , payer_address : Pubkey , recent_slot : Slot ,) -> (Instruction , Pubkey) { create_lookup_table_common (authority_address , payer_address , recent_slot , false) }
};
}
