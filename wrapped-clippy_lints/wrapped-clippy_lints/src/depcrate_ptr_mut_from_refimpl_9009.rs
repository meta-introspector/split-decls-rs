// Generated macro for impl_9009 (impl)
macro_rules! Depcrate_ptr_mut_from_refimpl_9009 {
() => {
// Module: crate::ptr::mut_from_ref
// Provides: {"impl_9009"}
// Dependencies: {}
impl < 'tcx > Visitor < 'tcx > for LifetimeVisitor < 'tcx > { fn visit_ty (& mut self , ty : & 'tcx hir :: Ty < 'tcx , hir :: AmbigArg >) { if let TyKind :: Ref (lt , ref m) = ty . kind { self . result . push ((lt , Some (m . mutbl) , ty . span)) ; } hir :: intravisit :: walk_ty (self , ty) ; } fn visit_generic_arg (& mut self , generic_arg : & 'tcx GenericArg < 'tcx >) { if let GenericArg :: Lifetime (lt) = generic_arg { self . result . push ((lt , None , generic_arg . span ())) ; } hir :: intravisit :: walk_generic_arg (self , generic_arg) ; } }
};
}
