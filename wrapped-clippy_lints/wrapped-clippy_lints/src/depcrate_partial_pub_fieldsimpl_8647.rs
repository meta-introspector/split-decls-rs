// Generated macro for impl_8647 (impl)
macro_rules! Depcrate_partial_pub_fieldsimpl_8647 {
() => {
// Module: crate::partial_pub_fields
// Provides: {"impl_8647"}
// Dependencies: {}
impl EarlyLintPass for PartialPubFields { fn check_item (& mut self , cx : & EarlyContext < '_ > , item : & Item) { let ItemKind :: Struct (_ , _ , ref st) = item . kind else { return ; } ; let mut fields = st . fields () . iter () ; let Some (first_field) = fields . next () else { return ; } ; let all_pub = first_field . vis . kind . is_pub () ; let all_priv = ! all_pub ; let msg = "mixed usage of pub and non-pub fields" ; for field in fields { if all_priv && field . vis . kind . is_pub () { # [expect (clippy :: collapsible_span_lint_calls , reason = "rust-clippy#7797")] span_lint_and_then (cx , PARTIAL_PUB_FIELDS , field . vis . span , msg , | diag | { diag . help ("consider using private field here") ; }) ; return ; } else if all_pub && ! field . vis . kind . is_pub () { # [expect (clippy :: collapsible_span_lint_calls , reason = "rust-clippy#7797")] span_lint_and_then (cx , PARTIAL_PUB_FIELDS , field . vis . span , msg , | diag | { diag . help ("consider using public field here") ; }) ; return ; } } } }
};
}
