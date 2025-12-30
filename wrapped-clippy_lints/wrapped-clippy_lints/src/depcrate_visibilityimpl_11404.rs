// Generated macro for impl_11404 (impl)
macro_rules! Depcrate_visibilityimpl_11404 {
() => {
// Module: crate::visibility
// Provides: {"impl_11404"}
// Dependencies: {}
impl EarlyLintPass for Visibility { fn check_item (& mut self , cx : & EarlyContext < '_ > , item : & Item) { if ! item . span . in_external_macro (cx . sess () . source_map ()) && let VisibilityKind :: Restricted { path , shorthand , .. } = & item . vis . kind { if * * path == kw :: SelfLower && ! is_from_proc_macro (cx , item . vis . span) { span_lint_and_then (cx , NEEDLESS_PUB_SELF , item . vis . span , format ! ("unnecessary `pub({}self)`" , if * shorthand { "" } else { "in " }) , | diag | { diag . span_suggestion_hidden (item . vis . span , "remove it" , String :: new () , Applicability :: MachineApplicable ,) ; } ,) ; } if (* * path == kw :: Super || * * path == kw :: SelfLower || * * path == kw :: Crate) && ! * shorthand && let [.. , last] = & * path . segments && ! is_from_proc_macro (cx , item . vis . span) { # [expect (clippy :: collapsible_span_lint_calls , reason = "rust-clippy#7797")] span_lint_and_then (cx , PUB_WITHOUT_SHORTHAND , item . vis . span , "usage of `pub` with `in`" , | diag | { diag . span_suggestion (item . vis . span , "remove it" , format ! ("pub({})" , last . ident) , Applicability :: MachineApplicable ,) ; } ,) ; } if * shorthand && let [.. , last] = & * path . segments && ! is_from_proc_macro (cx , item . vis . span) { # [expect (clippy :: collapsible_span_lint_calls , reason = "rust-clippy#7797")] span_lint_and_then (cx , PUB_WITH_SHORTHAND , item . vis . span , "usage of `pub` without `in`" , | diag | { diag . span_suggestion (item . vis . span , "add it" , format ! ("pub(in {})" , last . ident) , Applicability :: MachineApplicable ,) ; } ,) ; } } } }
};
}
