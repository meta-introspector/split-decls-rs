// Generated macro for GT_HELPER (static)
macro_rules! Depcrate_helpers_helper_extrasGT_HELPER {
() => {
// Module: crate::helpers::helper_extras
// Provides: {"GT_HELPER"}
// Dependencies: {}
pub (crate) static GT_HELPER : BinaryBoolHelper = BinaryBoolHelper { name : "gt" , op : | x , y | compare_json (x , y) == Some (Ordering :: Greater) , } ;
};
}
