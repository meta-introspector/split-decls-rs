// Generated macro for get_explicit_type (function)
macro_rules! Depcrate_transmute_transmute_ptr_to_refget_explicit_type {
() => {
// Module: crate::transmute::transmute_ptr_to_ref
// Provides: {"get_explicit_type"}
// Dependencies: {}
# [doc = " Gets the type `Bar` in `…::transmute<Foo, &Bar>`."] fn get_explicit_type < 'tcx > (path : & 'tcx Path < 'tcx >) -> Option < & 'tcx hir :: Ty < 'tcx > > { if let GenericArg :: Type (ty) = path . segments . last () ? . args ? . args . get (1) ? && let TyKind :: Ref (_ , ty) = & ty . kind { Some (ty . ty) } else { None } }
};
}
