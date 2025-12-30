// Generated macro for invoke_signed (function)
macro_rules! Depcrate_programinvoke_signed {
() => {
// Module: crate::program
// Provides: {"invoke_signed"}
// Dependencies: {}
# [doc = " Like [`solana_cpi::invoke_signed`], but with support"] # [doc = " for overwriting the `sol_invoke_signed` syscall stub."] # [doc = ""] # [doc = " [`solana_cpi::invoke_signed`]: https://docs.rs/solana-cpi/latest/solana_cpi/fn.invoke_signed.html"] pub fn invoke_signed (instruction : & Instruction , account_infos : & [AccountInfo] , signers_seeds : & [& [& [u8]]] ,) -> ProgramResult { for account_meta in instruction . accounts . iter () { for account_info in account_infos . iter () { if account_meta . pubkey == * account_info . key { if account_meta . is_writable { let _ = account_info . try_borrow_mut_lamports () ? ; let _ = account_info . try_borrow_mut_data () ? ; } else { let _ = account_info . try_borrow_lamports () ? ; let _ = account_info . try_borrow_data () ? ; } break ; } } } invoke_signed_unchecked (instruction , account_infos , signers_seeds) }
};
}
