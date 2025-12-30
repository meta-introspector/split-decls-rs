// Generated macro for invoke_unchecked (function)
macro_rules! Depcrateinvoke_unchecked {
() => {
// Module: crate
// Provides: {"invoke_unchecked"}
// Dependencies: {}
# [doc = " Invoke a cross-program instruction but don't enforce Rust's aliasing rules."] # [doc = ""] # [doc = " This function is like [`invoke`] except that it does not check that"] # [doc = " [`RefCell`]s within [`AccountInfo`]s are properly borrowable as described in"] # [doc = " the documentation for that function. Those checks consume CPU cycles that"] # [doc = " this function avoids."] # [doc = ""] # [doc = " [`RefCell`]: std::cell::RefCell"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " __This function is incorrectly missing an `unsafe` declaration.__"] # [doc = ""] # [doc = " If any of the writable accounts passed to the callee contain data that is"] # [doc = " borrowed within the calling program, and that data is written to by the"] # [doc = " callee, then Rust's aliasing rules will be violated and cause undefined"] # [doc = " behavior."] pub fn invoke_unchecked (instruction : & Instruction , account_infos : & [AccountInfo]) -> ProgramResult { invoke_signed_unchecked (instruction , account_infos , & []) }
};
}
