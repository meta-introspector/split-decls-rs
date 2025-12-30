// Generated macro for contains_param (function)
macro_rules! Depcrate_ty_type_certaintycontains_param {
() => {
// Module: crate::ty::type_certainty
// Provides: {"contains_param"}
// Dependencies: {}
fn contains_param (ty : Ty < '_ > , index : u32) -> bool { ty . walk () . any (| arg | matches ! (arg . kind () , GenericArgKind :: Type (ty) if ty . is_param (index))) }
};
}
