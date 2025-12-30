// Generated macro for macro_4337 (macro)
macro_rules! Depcrate_manual_option_as_slicemacro_4337 {
() => {
// Module: crate::manual_option_as_slice
// Provides: {"macro_4337"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " This detects various manual reimplementations of `Option::as_slice`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Those implementations are both more complex than calling `as_slice`"] # [doc = " and unlike that incur a branch, pessimizing performance and leading"] # [doc = " to more generated code."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = "# let opt = Some(1);"] # [doc = " _ = opt.as_ref().map_or(&[][..], std::slice::from_ref);"] # [doc = " _ = match opt.as_ref() {"] # [doc = "     Some(f) => std::slice::from_ref(f),"] # [doc = "     None => &[],"] # [doc = " };"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = "# let opt = Some(1);"] # [doc = " _ = opt.as_slice();"] # [doc = " _ = opt.as_slice();"] # [doc = " ```"] # [clippy :: version = "1.86.0"] pub MANUAL_OPTION_AS_SLICE , complexity , "manual `Option::as_slice`" }
};
}
