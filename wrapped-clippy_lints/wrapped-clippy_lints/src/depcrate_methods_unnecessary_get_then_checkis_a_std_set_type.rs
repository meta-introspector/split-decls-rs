// Generated macro for is_a_std_set_type (function)
macro_rules! Depcrate_methods_unnecessary_get_then_checkis_a_std_set_type {
() => {
// Module: crate::methods::unnecessary_get_then_check
// Provides: {"is_a_std_set_type"}
// Dependencies: {}
fn is_a_std_set_type (cx : & LateContext < '_ > , ty : Ty < '_ >) -> bool { ty . is_diag_item (cx , sym :: HashSet) || ty . is_diag_item (cx , sym :: BTreeSet) }
};
}
