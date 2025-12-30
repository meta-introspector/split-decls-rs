// Generated macro for ideal_unit_std (function)
macro_rules! Depcrate_displayideal_unit_std {
() => {
// Module: crate::display
// Provides: {"ideal_unit_std"}
// Dependencies: {}
# [cfg (feature = "std")] # [allow (dead_code)] fn ideal_unit_std (size : f64 , unit_base : f64) -> usize { assert ! (size . ln () >= unit_base , "only called when bytes >= unit") ; match (size . ln () / unit_base) as usize { 0 => unreachable ! () , e => e , } }
};
}
