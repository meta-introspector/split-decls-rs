// Generated macro for macro_1447 (macro)
macro_rules! Depcrate_dereferencemacro_1447 {
() => {
// Module: crate::dereference
// Provides: {"macro_1447"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for dereferencing expressions which would be covered by auto-deref."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This unnecessarily complicates the code."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let x = String::new();"] # [doc = " let y: &str = &*x;"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let x = String::new();"] # [doc = " let y: &str = &x;"] # [doc = " ```"] # [clippy :: version = "1.64.0"] pub EXPLICIT_AUTO_DEREF , complexity , "dereferencing when the compiler would automatically dereference" }
};
}
