// Generated macro for impl_836 (impl)
macro_rules! Depcrate_usageimpl_836 {
() => {
// Module: crate::usage
// Provides: {"impl_836"}
// Dependencies: {}
impl < 'tcx > Visitor < 'tcx > for ParamBindingIdCollector { fn visit_pat (& mut self , pat : & 'tcx hir :: Pat < 'tcx >) { if let hir :: PatKind :: Binding (_ , hir_id , ..) = pat . kind { self . binding_hir_ids . push (hir_id) ; } intravisit :: walk_pat (self , pat) ; } }
};
}
