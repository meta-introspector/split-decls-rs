// Generated macro for macro_8905 (macro)
macro_rules! Depcrate_pass_by_ref_or_valuemacro_8905 {
() => {
// Module: crate::pass_by_ref_or_value
// Provides: {"macro_8905"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for functions taking arguments by value, where"] # [doc = " the argument type is `Copy` and large enough to be worth considering"] # [doc = " passing by reference. Does not trigger if the function is being exported,"] # [doc = " because that might induce API breakage, if the parameter is declared as mutable,"] # [doc = " or if the argument is a `self`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Arguments passed by value might result in an unnecessary"] # [doc = " shallow copy, taking up more space in the stack and requiring a call to"] # [doc = " `memcpy`, which can be expensive."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " #[derive(Clone, Copy)]"] # [doc = " struct TooLarge([u8; 2048]);"] # [doc = ""] # [doc = " fn foo(v: TooLarge) {}"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # #[derive(Clone, Copy)]"] # [doc = " # struct TooLarge([u8; 2048]);"] # [doc = " fn foo(v: &TooLarge) {}"] # [doc = " ```"] # [clippy :: version = "1.49.0"] pub LARGE_TYPES_PASSED_BY_VALUE , pedantic , "functions taking large arguments by value" }
};
}
