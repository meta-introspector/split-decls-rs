// Generated macro for impl_72 (impl)
macro_rules! Depcrate_visit_traversalimpl_72 {
() => {
// Module: crate::visit::traversal
// Provides: {"impl_72"}
// Dependencies: {}
impl < C , W : ? Sized > Walker < C > for & mut W where W : Walker < C > , { type Item = W :: Item ; fn walk_next (& mut self , context : C) -> Option < Self :: Item > { (* * self) . walk_next (context) } }
};
}
