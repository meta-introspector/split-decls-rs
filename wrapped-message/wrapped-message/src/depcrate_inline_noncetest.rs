// Generated macro for test (module)
macro_rules! Depcrate_inline_noncetest {
() => {
// Module: crate::inline_nonce
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use { super :: * , solana_system_interface :: instruction :: { advance_nonce_account , SystemInstruction } , } ; # [test] fn inline_instruction_data_matches_program () { let nonce = Address :: new_unique () ; let nonce_authority = Address :: new_unique () ; assert_eq ! (advance_nonce_account_instruction (& nonce , & nonce_authority) , advance_nonce_account (& nonce , & nonce_authority) ,) ; } # [test] fn test_advance_nonce_ix_prefix () { let advance_nonce_ix : SystemInstruction = bincode :: deserialize (& ADVANCE_NONCE_DATA) . unwrap () ; assert_eq ! (advance_nonce_ix , SystemInstruction :: AdvanceNonceAccount) ; } }
};
}
