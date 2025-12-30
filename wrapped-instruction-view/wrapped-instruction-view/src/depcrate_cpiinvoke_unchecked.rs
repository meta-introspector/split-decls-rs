// Generated macro for invoke_unchecked (function)
macro_rules! Depcrate_cpiinvoke_unchecked {
() => {
// Module: crate::cpi
// Provides: {"invoke_unchecked"}
// Dependencies: {}
# [doc = " Invoke a cross-program instruction but don't enforce Rust's aliasing rules."] # [doc = ""] # [doc = " This function does not check that [`CpiAccount`]s are properly borrowable."] # [doc = " Those checks consume CUs that this function avoids."] # [doc = ""] # [doc = " Note that the maximum number of accounts that can be passed to a cross-program"] # [doc = " invocation is defined by the `MAX_CPI_ACCOUNTS` constant. Even if the `[CpiAccount]`"] # [doc = " slice has more accounts, only the number of accounts required by the `instruction`"] # [doc = " will be used."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " If any of the writable accounts passed to the callee contain data that is"] # [doc = " borrowed within the calling program, and that data is written to by the"] # [doc = " callee, then Rust's aliasing rules will be violated and cause undefined"] # [doc = " behavior."] # [inline (always)] pub unsafe fn invoke_unchecked (instruction : & InstructionView , accounts : & [CpiAccount]) { invoke_signed_unchecked (instruction , accounts , & []) }
};
}
