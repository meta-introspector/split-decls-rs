// Generated macro for macro_100 (macro)
macro_rules! Depcrate_lint_without_lint_passmacro_100 {
() => {
// Module: crate::lint_without_lint_pass
// Provides: {"macro_100"}
// Dependencies: {}
declare_tool_lint ! { # [doc = " ### What it does"] # [doc = " Ensures every lint is associated to a `LintPass`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " The compiler only knows lints via a `LintPass`. Without"] # [doc = " putting a lint to a `LintPass::lint_vec()`'s return, the compiler will not"] # [doc = " know the name of the lint."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " Only checks for lints associated using the `declare_lint_pass!` and"] # [doc = " `impl_lint_pass!` macros."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " declare_lint! { pub LINT_1, ... }"] # [doc = " declare_lint! { pub LINT_2, ... }"] # [doc = " declare_lint! { pub FORGOTTEN_LINT, ... }"] # [doc = " // ..."] # [doc = " declare_lint_pass!(Pass => [LINT_1, LINT_2]);"] # [doc = " // missing FORGOTTEN_LINT"] # [doc = " ```"] pub clippy :: LINT_WITHOUT_LINT_PASS , Warn , "declaring a lint without associating it in a LintPass" , report_in_external_macro : true }
};
}
