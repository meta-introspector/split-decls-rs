// Generated macro for impl_8815 (impl)
macro_rules! Depcrate_ptrimpl_8815 {
() => {
// Module: crate::ptr
// Provides: {"impl_8815"}
// Dependencies: {}
impl < 'tcx > Visitor < 'tcx > for LifetimeVisitor < 'tcx > { fn visit_ty (& mut self , ty : & 'tcx hir :: Ty < 'tcx , hir :: AmbigArg >) { if let TyKind :: Ref (lt , ref m) = ty . kind { self . result . push ((lt , Some (m . mutbl) , ty . span)) ; } hir :: intravisit :: walk_ty (self , ty) ; } fn visit_generic_arg (& mut self , generic_arg : & 'tcx GenericArg < 'tcx >) { if let GenericArg :: Lifetime (lt) = generic_arg { self . result . push ((lt , None , generic_arg . span ())) ; } hir :: intravisit :: walk_generic_arg (self , generic_arg) ; } }
};
}
