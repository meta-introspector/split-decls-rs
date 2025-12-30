// Generated macro for macro_52 (macro)
macro_rules! Depcrate_utils_authormacro_52 {
() => {
// Module: crate::utils::author
// Provides: {"macro_52"}
// Dependencies: {}
declare_lint_pass ! (# [doc = " ### What it does"] # [doc = " Generates clippy code that detects the offending pattern"] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " // ./tests/ui/my_lint.rs"] # [doc = " fn foo() {"] # [doc = "     // detect the following pattern"] # [doc = "     #[clippy::author]"] # [doc = "     if x == 42 {"] # [doc = "         // but ignore everything from here on"] # [doc = "         #![clippy::author = \"ignore\"]"] # [doc = "     }"] # [doc = "     ()"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Running `TESTNAME=ui/my_lint cargo uitest` will produce"] # [doc = " a `./tests/ui/new_lint.stdout` file with the generated code:"] # [doc = ""] # [doc = " ```rust,ignore"] # [doc = " // ./tests/ui/new_lint.stdout"] # [doc = " if ExprKind::If(ref cond, ref then, None) = item.kind"] # [doc = "     && let ExprKind::Binary(BinOp::Eq, ref left, ref right) = cond.kind"] # [doc = "     && let ExprKind::Path(ref path) = left.kind"] # [doc = "     && let ExprKind::Lit(ref lit) = right.kind"] # [doc = "     && let LitKind::Int(42, _) = lit.node"] # [doc = " {"] # [doc = "     // report your lint here"] # [doc = " }"] # [doc = " ```"] Author => []) ;
};
}
