// Generated macro for compare_json (function)
macro_rules! Depcrate_helpers_helper_extrascompare_json {
() => {
// Module: crate::helpers::helper_extras
// Provides: {"compare_json"}
// Dependencies: {}
fn compare_json (x : & Json , y : & Json) -> Option < Ordering > { fn cmp_num_str (a_num : & serde_json :: Number , b_str : & str) -> Option < Ordering > { let b_num = serde_json :: Number :: from_str (b_str) . ok () ? ; cmp_nums (a_num , & b_num) } fn cmp_nums (a_num : & serde_json :: Number , b_num : & serde_json :: Number) -> Option < Ordering > { if a_num . is_u64 () { let a = a_num . as_u64 () ? ; if b_num . is_u64 () { NumOrd :: num_partial_cmp (& a , & b_num . as_u64 () ?) } else if b_num . is_i64 () { NumOrd :: num_partial_cmp (& a , & b_num . as_i64 () ?) } else { NumOrd :: num_partial_cmp (& a , & b_num . as_f64 () ?) } } else if a_num . is_i64 () { let a = a_num . as_i64 () ? ; if b_num . is_u64 () { NumOrd :: num_partial_cmp (& a , & b_num . as_u64 () ?) } else if b_num . is_i64 () { NumOrd :: num_partial_cmp (& a , & b_num . as_i64 () ?) } else { NumOrd :: num_partial_cmp (& a , & b_num . as_f64 () ?) } } else { let a = a_num . as_f64 () ? ; if b_num . is_u64 () { NumOrd :: num_partial_cmp (& a , & b_num . as_u64 () ?) } else if b_num . is_i64 () { NumOrd :: num_partial_cmp (& a , & b_num . as_i64 () ?) } else { NumOrd :: num_partial_cmp (& a , & b_num . as_f64 () ?) } } } match (x , y) { (Json :: Number (a) , Json :: Number (b)) => cmp_nums (a , b) , (Json :: String (a) , Json :: String (b)) => Some (a . cmp (b)) , (Json :: Bool (a) , Json :: Bool (b)) => Some (a . cmp (b)) , (Json :: Number (a) , Json :: String (b)) => cmp_num_str (a , b) , (Json :: String (a) , Json :: Number (b)) => cmp_num_str (b , a) . map (Ordering :: reverse) , _ => None , } }
};
}
