// Generated macro for macro_1420 (macro)
macro_rules! Depcrate_dereferencemacro_1420 {
() => {
// Module: crate::dereference
// Provides: {"macro_1420"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for `ref` bindings which create a reference to a reference."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " The address-of operator at the use site is clearer about the need for a reference."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let x = Some(\"\");"] # [doc = " if let Some(ref x) = x {"] # [doc = "     // use `x` here"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let x = Some(\"\");"] # [doc = " if let Some(x) = x {"] # [doc = "     // use `&x` here"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.54.0"] pub REF_BINDING_TO_REFERENCE , pedantic , "`ref` binding to a reference" }
};
}
