// Generated macro for collect_generic_params (function)
macro_rules! Depcrate_non_send_fields_in_send_tycollect_generic_params {
() => {
// Module: crate::non_send_fields_in_send_ty
// Provides: {"collect_generic_params"}
// Dependencies: {}
# [doc = " Given a type, collect all of its generic parameters."] # [doc = " Example: `MyStruct<P, Box<Q, R>>` => `vec![P, Q, R]`"] fn collect_generic_params (ty : Ty < '_ >) -> Vec < Ty < '_ > > { ty . walk () . filter_map (| inner | match inner . kind () { GenericArgKind :: Type (inner_ty) => Some (inner_ty) , _ => None , }) . filter (| & inner_ty | is_ty_param (inner_ty)) . collect () }
};
}
