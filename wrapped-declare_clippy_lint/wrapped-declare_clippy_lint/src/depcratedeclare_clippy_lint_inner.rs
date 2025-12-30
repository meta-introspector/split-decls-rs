// Generated macro for declare_clippy_lint_inner (macro)
macro_rules! Depcratedeclare_clippy_lint_inner {
() => {
// Module: crate
// Provides: {"declare_clippy_lint_inner"}
// Dependencies: {}
# [macro_export] macro_rules ! declare_clippy_lint_inner { ($ (# [doc = $ docs : literal]) * # [clippy :: version = $ version : literal] $ vis : vis $ lint_name : ident , $ level : ident , $ category : ident , $ desc : literal $ (, @ eval_always = $ eval_always : literal) ?) => { $ crate :: rustc_session :: declare_tool_lint ! { $ (# [doc = $ docs]) * # [clippy :: version = $ version] $ vis clippy ::$ lint_name , $ level , $ desc , report_in_external_macro : true $ (, @ eval_always = $ eval_always) ? } pub (crate) static $ { concat ($ lint_name , _INFO) } : &'static $ crate :: LintInfo = &$ crate :: LintInfo { lint : $ lint_name , category : $ crate :: LintCategory ::$ category , explanation : concat ! ($ ($ docs , "\n" ,) *) , location : concat ! (file ! () , "#L" , line ! ()) , version : $ version , } ; } ; }
};
}
