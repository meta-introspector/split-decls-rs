// Generated macro for macro_10437 (macro)
macro_rules! Depcrate_transmutemacro_10437 {
() => {
// Module: crate::transmute
// Provides: {"macro_10437"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for transmutes that can't ever be correct on any"] # [doc = " architecture."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " It's basically guaranteed to be undefined behavior."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " When accessing C, users might want to store pointer"] # [doc = " sized objects in `extradata` arguments to save an allocation."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```ignore"] # [doc = " let ptr: *const T = core::intrinsics::transmute('x')"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub WRONG_TRANSMUTE , correctness , "transmutes that are confusing at best, undefined behavior at worst and always useless" }
};
}
