// Generated macro for impl_487 (impl)
macro_rules! Depcrate_repository_revision_explainimpl_487 {
() => {
// Module: crate::repository::revision::explain
// Provides: {"impl_487"}
// Dependencies: {}
impl Delegate for Explain < '_ > { fn done (& mut self) { if ! self . has_implicit_anchor && self . ref_name . is_none () && self . oid_prefix . is_none () { self . err = Some ("Incomplete specification lacks its anchor, like a reference or object name" . into ()) ; } } }
};
}
