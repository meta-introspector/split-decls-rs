// Generated macro for try_resolve_type (function)
macro_rules! Depcrate_implied_bounds_in_implstry_resolve_type {
() => {
// Module: crate::implied_bounds_in_impls
// Provides: {"try_resolve_type"}
// Dependencies: {}
# [doc = " Tries to \"resolve\" a type."] # [doc = " The index passed to this function must start with `Self=0`, i.e. it must be a valid"] # [doc = " type parameter index."] # [doc = " If the index is out of bounds, it means that the generic parameter has a default type."] fn try_resolve_type < 'tcx > (tcx : TyCtxt < 'tcx > , args : & 'tcx [GenericArg < 'tcx >] , generics : & 'tcx Generics , index : usize ,) -> Option < Ty < 'tcx > > { match args . get (index - 1) { Some (GenericArg :: Type (ty)) => Some (lower_ty (tcx , ty . as_unambig_ty ())) , Some (_) => None , None => Some (tcx . type_of (generics . own_params [index] . def_id) . skip_binder ()) , } }
};
}
