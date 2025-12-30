// Generated macro for invoke (function)
macro_rules! Depcrate_programinvoke {
() => {
// Module: crate::program
// Provides: {"invoke"}
// Dependencies: {}
# [doc = " Like [`solana_cpi::invoke`], but with support"] # [doc = " for overwriting the `sol_invoke_signed` syscall stub."] # [doc = ""] # [doc = " [`solana_cpi::invoke`]: https://docs.rs/solana-cpi/latest/solana_cpi/fn.invoke.html"] pub fn invoke (instruction : & Instruction , account_infos : & [AccountInfo]) -> ProgramResult { invoke_signed (instruction , account_infos , & []) }
};
}
