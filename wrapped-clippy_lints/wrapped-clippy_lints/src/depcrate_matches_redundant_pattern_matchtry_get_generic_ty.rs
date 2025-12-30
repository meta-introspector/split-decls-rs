// Generated macro for try_get_generic_ty (function)
macro_rules! Depcrate_matches_redundant_pattern_matchtry_get_generic_ty {
() => {
// Module: crate::matches::redundant_pattern_match
// Provides: {"try_get_generic_ty"}
// Dependencies: {}
fn try_get_generic_ty (ty : Ty < '_ > , index : usize) -> Option < Ty < '_ > > { if let ty :: Adt (_ , subs) = ty . kind () && let Some (sub) = subs . get (index) && let GenericArgKind :: Type (sub_ty) = sub . kind () { Some (sub_ty) } else { None } }
};
}
