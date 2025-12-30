// Generated macro for min_non_zero_cap (function)
macro_rules! Depcrate_raw_vecmin_non_zero_cap {
() => {
// Module: crate::raw_vec
// Provides: {"min_non_zero_cap"}
// Dependencies: {}
const fn min_non_zero_cap (size : usize) -> usize { if size == 1 { 8 } else if size <= 1024 { 4 } else { 1 } }
};
}
