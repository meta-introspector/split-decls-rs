// Generated macro for invoke_unchecked (function)
macro_rules! Depcrate_programinvoke_unchecked {
() => {
// Module: crate::program
// Provides: {"invoke_unchecked"}
// Dependencies: {}
# [doc = " Like [`solana_cpi::invoke_unchecked`], but with support"] # [doc = " for overwriting the `sol_invoke_signed` syscall stub."] # [doc = ""] # [doc = " [`solana_cpi::invoke_unchecked`]: https://docs.rs/solana-cpi/latest/solana_cpi/fn.invoke_unchecked.html"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " __This function is incorrectly missing an `unsafe` declaration.__"] # [doc = ""] # [doc = " If any of the writable accounts passed to the callee contain data that is"] # [doc = " borrowed within the calling program, and that data is written to by the"] # [doc = " callee, then Rust's aliasing rules will be violated and cause undefined"] # [doc = " behavior."] pub fn invoke_unchecked (instruction : & Instruction , account_infos : & [AccountInfo]) -> ProgramResult { invoke_signed_unchecked (instruction , account_infos , & []) }
};
}
