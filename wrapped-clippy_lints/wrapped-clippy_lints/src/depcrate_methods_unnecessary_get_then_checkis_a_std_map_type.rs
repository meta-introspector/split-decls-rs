// Generated macro for is_a_std_map_type (function)
macro_rules! Depcrate_methods_unnecessary_get_then_checkis_a_std_map_type {
() => {
// Module: crate::methods::unnecessary_get_then_check
// Provides: {"is_a_std_map_type"}
// Dependencies: {}
fn is_a_std_map_type (cx : & LateContext < '_ > , ty : Ty < '_ >) -> bool { ty . is_diag_item (cx , sym :: HashMap) || ty . is_diag_item (cx , sym :: BTreeMap) }
};
}
