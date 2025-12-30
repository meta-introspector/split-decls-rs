// Generated macro for GTE_HELPER (static)
macro_rules! Depcrate_helpers_helper_extrasGTE_HELPER {
() => {
// Module: crate::helpers::helper_extras
// Provides: {"GTE_HELPER"}
// Dependencies: {}
pub (crate) static GTE_HELPER : BinaryBoolHelper = BinaryBoolHelper { name : "gte" , op : | x , y | compare_json (x , y) . is_some_and (| ord | ord != Ordering :: Less) , } ;
};
}
