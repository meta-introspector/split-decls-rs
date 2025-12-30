// Generated macro for macro_10110 (macro)
macro_rules! Depcrate_transmutemacro_10110 {
() => {
// Module: crate::transmute
// Provides: {"macro_10110"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for transmute calls which would receive a null pointer."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Transmuting a null pointer is undefined behavior."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " Not all cases can be detected at the moment of this writing."] # [doc = " For example, variables which hold a null pointer and are then fed to a `transmute`"] # [doc = " call, aren't detectable yet."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let null_ref: &u64 = unsafe { std::mem::transmute(0 as *const u64) };"] # [doc = " ```"] # [clippy :: version = "1.35.0"] pub TRANSMUTING_NULL , correctness , "transmutes from a null pointer to a reference, which is undefined behavior" }
};
}
