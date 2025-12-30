// Generated macro for folded_multiply (function)
macro_rules! Depcrate_operationsfolded_multiply {
() => {
// Module: crate::operations
// Provides: {"folded_multiply"}
// Dependencies: {}
# [inline (always)] # [cfg (not (folded_multiply))] pub (crate) const fn folded_multiply (s : u64 , by : u64) -> u64 { let b1 = s . wrapping_mul (by . swap_bytes ()) ; let b2 = s . swap_bytes () . wrapping_mul (! by) ; b1 ^ b2 . swap_bytes () }
};
}
