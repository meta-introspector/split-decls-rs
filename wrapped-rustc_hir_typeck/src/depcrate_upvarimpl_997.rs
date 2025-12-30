// Generated macro for impl_997 (impl)
macro_rules! Depcrate_upvarimpl_997 {
() => {
// Module: crate::upvar
// Provides: {"impl_997"}
// Dependencies: {}
impl < 'a , 'tcx > FnCtxt < 'a , 'tcx > { pub (crate) fn closure_analyze (& self , body : & 'tcx hir :: Body < 'tcx >) { InferBorrowKindVisitor { fcx : self } . visit_body (body) ; assert ! (self . deferred_call_resolutions . borrow () . is_empty ()) ; } }
};
}
