// Generated macro for round_mut_ptr_up_to_unchecked (function)
macro_rules! Depcrateround_mut_ptr_up_to_unchecked {
() => {
// Module: crate
// Provides: {"round_mut_ptr_up_to_unchecked"}
// Dependencies: {}
# [inline] pub (crate) unsafe fn round_mut_ptr_up_to_unchecked (ptr : * mut u8 , divisor : usize) -> * mut u8 { debug_assert ! (divisor > 0) ; debug_assert ! (divisor . is_power_of_two ()) ; let aligned = round_up_to_unchecked (ptr as usize , divisor) ; let delta = aligned - (ptr as usize) ; ptr . add (delta) }
};
}
