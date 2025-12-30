// Generated macro for align_to_power_of2 (function)
macro_rules! Depcrate_math_extrasalign_to_power_of2 {
() => {
// Module: crate::math_extras
// Provides: {"align_to_power_of2"}
// Dependencies: {}
pub (crate) const fn align_to_power_of2 (value : u64 , align : u64) -> u64 { assert ! (align != 0 && (align & (align - 1)) == 0 , "Align must be a power of 2") ; let neg_align = align . wrapping_neg () ; (value + align - 1) & neg_align }
};
}
