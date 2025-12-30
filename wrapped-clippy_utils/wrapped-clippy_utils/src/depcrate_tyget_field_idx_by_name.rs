// Generated macro for get_field_idx_by_name (function)
macro_rules! Depcrate_tyget_field_idx_by_name {
() => {
// Module: crate::ty
// Provides: {"get_field_idx_by_name"}
// Dependencies: {}
pub fn get_field_idx_by_name (ty : Ty < '_ > , name : Symbol) -> Option < usize > { match * ty . kind () { ty :: Adt (def , _) if def . is_union () || def . is_struct () => { def . non_enum_variant () . fields . iter () . position (| f | f . name == name) } , ty :: Tuple (_) => name . as_str () . parse :: < usize > () . ok () , _ => None , } }
};
}
