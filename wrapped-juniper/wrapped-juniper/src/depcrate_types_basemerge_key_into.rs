// Generated macro for merge_key_into (function)
macro_rules! Depcrate_types_basemerge_key_into {
() => {
// Module: crate::types::base
// Provides: {"merge_key_into"}
// Dependencies: {}
# [doc = " Merges `response_name`/`value` pair into `result`"] pub (crate) fn merge_key_into < S > (result : & mut Object < S > , response_name : & str , value : Value < S >) { if let Some (v) = result . get_mut_field_value (response_name) { match v { Value :: Object (dest_obj) => { if let Value :: Object (src_obj) = value { merge_maps (dest_obj , src_obj) ; } } Value :: List (dest_list) => { if let Value :: List (src_list) = value { dest_list . iter_mut () . zip (src_list) . for_each (| (d , s) | { if let Value :: Object (d_obj) = d { if let Value :: Object (s_obj) = s { merge_maps (d_obj , s_obj) ; } } }) ; } } _ => { } } return ; } result . add_field (response_name , value) ; }
};
}
