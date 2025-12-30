// Generated macro for impl_9088 (impl)
macro_rules! Depcrate_pub_useimpl_9088 {
() => {
// Module: crate::pub_use
// Provides: {"impl_9088"}
// Dependencies: {}
impl EarlyLintPass for PubUse { fn check_item (& mut self , cx : & EarlyContext < '_ > , item : & Item) { if let ItemKind :: Use (_) = item . kind && let VisibilityKind :: Public = item . vis . kind { # [expect (clippy :: collapsible_span_lint_calls , reason = "rust-clippy#7797")] span_lint_and_then (cx , PUB_USE , item . span , "using `pub use`" , | diag | { diag . help ("move the exported item to a public module instead") ; }) ; } } }
};
}
