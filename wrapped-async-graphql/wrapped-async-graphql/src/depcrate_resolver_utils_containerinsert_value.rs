// Generated macro for insert_value (function)
macro_rules! Depcrate_resolver_utils_containerinsert_value {
() => {
// Module: crate::resolver_utils::container
// Provides: {"insert_value"}
// Dependencies: {}
fn insert_value (target : & mut IndexMap < Name , Value > , name : Name , value : Value) { if let Some (prev_value) = target . get_mut (& name) { if let Value :: Object (target_map) = prev_value { if let Value :: Object (obj) = value { for (key , value) in obj . into_iter () { insert_value (target_map , key , value) ; } } } else if let Value :: List (target_list) = prev_value { if let Value :: List (list) = value { for (idx , value) in list . into_iter () . enumerate () { if let Some (Value :: Object (target_map)) = target_list . get_mut (idx) { if let Value :: Object (obj) = value { for (key , value) in obj . into_iter () { insert_value (target_map , key , value) ; } } } } } } } else { target . insert (name , value) ; } }
};
}
