// Generated macro for LTE_HELPER (static)
macro_rules! Depcrate_helpers_helper_extrasLTE_HELPER {
() => {
// Module: crate::helpers::helper_extras
// Provides: {"LTE_HELPER"}
// Dependencies: {}
pub (crate) static LTE_HELPER : BinaryBoolHelper = BinaryBoolHelper { name : "lte" , op : | x , y | compare_json (x , y) . is_some_and (| ord | ord != Ordering :: Greater) , } ;
};
}
