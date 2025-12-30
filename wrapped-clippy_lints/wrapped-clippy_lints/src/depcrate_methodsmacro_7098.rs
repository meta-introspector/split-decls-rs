// Generated macro for macro_7098 (macro)
macro_rules! Depcrate_methodsmacro_7098 {
() => {
// Module: crate::methods
// Provides: {"macro_7098"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for string slices immediately followed by `as_bytes`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " It involves doing an unnecessary UTF-8 alignment check which is less efficient, and can cause a panic."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " In some cases, the UTF-8 validation and potential panic from string slicing may be required for"] # [doc = " the code's correctness. If you need to ensure the slice boundaries fall on valid UTF-8 character"] # [doc = " boundaries, the original form (`s[1..5].as_bytes()`) should be preferred."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust"] # [doc = " let s = \"Lorem ipsum\";"] # [doc = " s[1..5].as_bytes();"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```rust"] # [doc = " let s = \"Lorem ipsum\";"] # [doc = " &s.as_bytes()[1..5];"] # [doc = " ```"] # [clippy :: version = "1.86.0"] pub SLICED_STRING_AS_BYTES , perf , "slicing a string and immediately calling as_bytes is less efficient and can lead to panics" }
};
}
