// Generated macro for round_mut_ptr_down_to (function)
macro_rules! Depcrateround_mut_ptr_down_to {
() => {
// Module: crate
// Provides: {"round_mut_ptr_down_to"}
// Dependencies: {}
# [doc = " Same as `round_down_to` but preserves pointer provenance."] # [inline] pub (crate) fn round_mut_ptr_down_to (ptr : * mut u8 , divisor : usize) -> * mut u8 { debug_assert ! (divisor > 0) ; debug_assert ! (divisor . is_power_of_two ()) ; ptr . wrapping_sub (ptr as usize & (divisor - 1)) }
};
}
