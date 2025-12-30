// Generated macro for impl_10129 (impl)
macro_rules! Depcrate_tabs_in_doc_commentsimpl_10129 {
() => {
// Module: crate::tabs_in_doc_comments
// Provides: {"impl_10129"}
// Dependencies: {}
impl EarlyLintPass for TabsInDocComments { fn check_attribute (& mut self , cx : & EarlyContext < '_ > , attribute : & ast :: Attribute) { Self :: warn_if_tabs_in_doc (cx , attribute) ; } }
};
}
