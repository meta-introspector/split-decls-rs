// Generated macro for impl_2700 (impl)
macro_rules! Depcrate_implicit_hasherimpl_2700 {
() => {
// Module: crate::implicit_hasher
// Provides: {"impl_2700"}
// Dependencies: {}
impl < 'tcx > Visitor < 'tcx > for ImplicitHasherTypeVisitor < '_ , 'tcx > { fn visit_ty (& mut self , t : & 'tcx hir :: Ty < '_ , AmbigArg >) { if let Some (target) = ImplicitHasherType :: new (self . cx , t . as_unambig_ty ()) { self . found . push (target) ; } walk_ty (self , t) ; } }
};
}
