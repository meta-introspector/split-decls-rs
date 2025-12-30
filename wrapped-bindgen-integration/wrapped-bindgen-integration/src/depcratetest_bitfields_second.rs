// Generated macro for test_bitfields_second (function)
macro_rules! Depcratetest_bitfields_second {
() => {
// Module: crate
// Provides: {"test_bitfields_second"}
// Dependencies: {}
# [test] fn test_bitfields_second () { let mut second : bindings :: bitfields :: Second = unsafe { mem :: zeroed () } ; assert ! (unsafe { second . assert (0 , false) }) ; second . set_thirty_one_bits (1337) ; second . set_one_bit (true) ; assert ! (unsafe { second . assert (1337 , true) }) ; }
};
}
