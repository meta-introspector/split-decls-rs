// Generated macro for LT_HELPER (static)
macro_rules! Depcrate_helpers_helper_extrasLT_HELPER {
() => {
// Module: crate::helpers::helper_extras
// Provides: {"LT_HELPER"}
// Dependencies: {}
pub (crate) static LT_HELPER : BinaryBoolHelper = BinaryBoolHelper { name : "lt" , op : | x , y | compare_json (x , y) == Some (Ordering :: Less) , } ;
};
}
