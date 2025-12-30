// Generated macro for macro_10111 (macro)
macro_rules! Depcrate_transmutemacro_10111 {
() => {
// Module: crate::transmute
// Provides: {"macro_10111"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for null function pointer creation through transmute."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Creating a null function pointer is undefined behavior."] # [doc = ""] # [doc = " More info: https://doc.rust-lang.org/nomicon/ffi.html#the-nullable-pointer-optimization"] # [doc = ""] # [doc = " ### Known problems"] # [doc = " Not all cases can be detected at the moment of this writing."] # [doc = " For example, variables which hold a null pointer and are then fed to a `transmute`"] # [doc = " call, aren't detectable yet."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let null_fn: fn() = unsafe { std::mem::transmute( std::ptr::null::<()>() ) };"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let null_fn: Option<fn()> = None;"] # [doc = " ```"] # [clippy :: version = "1.68.0"] pub TRANSMUTE_NULL_TO_FN , correctness , "transmute results in a null function pointer, which is undefined behavior" }
};
}
