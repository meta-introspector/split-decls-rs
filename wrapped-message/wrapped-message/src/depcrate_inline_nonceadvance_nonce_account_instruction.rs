// Generated macro for advance_nonce_account_instruction (function)
macro_rules! Depcrate_inline_nonceadvance_nonce_account_instruction {
() => {
// Module: crate::inline_nonce
// Provides: {"advance_nonce_account_instruction"}
// Dependencies: {}
# [doc = " Inlined `advance_nonce_account` instruction creator to avoid"] # [doc = " solana_system_interface and bincode deps"] pub (crate) fn advance_nonce_account_instruction (nonce_pubkey : & Address , nonce_authority_pubkey : & Address ,) -> Instruction { Instruction :: new_with_bytes (system_program :: id () , & ADVANCE_NONCE_DATA , vec ! [AccountMeta :: new (* nonce_pubkey , false) , # [allow (deprecated)] AccountMeta :: new_readonly (sysvar :: recent_blockhashes :: id () , false) , AccountMeta :: new_readonly (* nonce_authority_pubkey , true) ,] ,) }
};
}
