// Generated macro for verify_nonce_account (function)
macro_rules! Depcrateverify_nonce_account {
() => {
// Module: crate
// Provides: {"verify_nonce_account"}
// Dependencies: {}
# [doc = " Checks if the recent_blockhash field in Transaction verifies, and returns"] # [doc = " nonce account data if so."] pub fn verify_nonce_account (account : & AccountSharedData , recent_blockhash : & Hash ,) -> Option < Data > { (account . owner () == & system_program :: id ()) . then (| | { StateMut :: < Versions > :: state (account) . ok () ? . verify_recent_blockhash (recent_blockhash) . cloned () }) . flatten () }
};
}
