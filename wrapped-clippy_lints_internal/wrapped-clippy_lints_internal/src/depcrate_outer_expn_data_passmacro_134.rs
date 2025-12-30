// Generated macro for macro_134 (macro)
macro_rules! Depcrate_outer_expn_data_passmacro_134 {
() => {
// Module: crate::outer_expn_data_pass
// Provides: {"macro_134"}
// Dependencies: {}
declare_tool_lint ! { # [doc = " ### What it does"] # [doc = " Checks for calls to `cx.outer().expn_data()` and suggests to use"] # [doc = " the `cx.outer_expn_data()`"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " `cx.outer_expn_data()` is faster and more concise."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " expr.span.ctxt().outer().expn_data()"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```rust,ignore"] # [doc = " expr.span.ctxt().outer_expn_data()"] # [doc = " ```"] pub clippy :: OUTER_EXPN_EXPN_DATA , Warn , "using `cx.outer_expn().expn_data()` instead of `cx.outer_expn_data()`" , report_in_external_macro : true }
};
}
