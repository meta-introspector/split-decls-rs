// Generated macro for macro_97 (macro)
macro_rules! Depcrate_lint_without_lint_passmacro_97 {
() => {
// Module: crate::lint_without_lint_pass
// Provides: {"macro_97"}
// Dependencies: {}
declare_tool_lint ! { # [doc = " ### What it does"] # [doc = " Checks for cases of an auto-generated lint without an updated description,"] # [doc = " i.e. `default lint description`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Indicates that the lint is not finished."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " declare_lint! { pub COOL_LINT, nursery, \"default lint description\" }"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```rust,ignore"] # [doc = " declare_lint! { pub COOL_LINT, nursery, \"a great new lint\" }"] # [doc = " ```"] pub clippy :: DEFAULT_LINT , Warn , "found 'default lint description' in a lint declaration" , report_in_external_macro : true }
};
}
