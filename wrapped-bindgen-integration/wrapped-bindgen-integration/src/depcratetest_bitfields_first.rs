// Generated macro for test_bitfields_first (function)
macro_rules! Depcratetest_bitfields_first {
() => {
// Module: crate
// Provides: {"test_bitfields_first"}
// Dependencies: {}
# [test] fn test_bitfields_first () { let mut first : bindings :: bitfields :: First = unsafe { mem :: zeroed () } ; assert ! (unsafe { first . assert (0 , 0 , 0) }) ; first . set_three_bits_byte_one (2) ; first . set_six_bits_byte_two (42) ; first . set_two_bits_byte_two (1) ; assert ! (unsafe { first . assert (2 , 42 , 1) }) ; }
};
}
