// Generated macro for ideal_unit_no_std (function)
macro_rules! Depcrate_displayideal_unit_no_std {
() => {
// Module: crate::display
// Provides: {"ideal_unit_no_std"}
// Dependencies: {}
# [allow (dead_code)] fn ideal_unit_no_std (size : f64 , unit : u64) -> usize { assert ! (size >= unit as f64 , "only called when bytes >= unit") ; let mut ideal_prefix = 0 ; let mut ideal_size = size ; loop { ideal_prefix += 1 ; ideal_size /= unit as f64 ; if ideal_size < unit as f64 { break ; } } ideal_prefix }
};
}
