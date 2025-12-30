// Generated macro for test_bitfields_fifth (function)
macro_rules! Depcratetest_bitfields_fifth {
() => {
// Module: crate
// Provides: {"test_bitfields_fifth"}
// Dependencies: {}
# [test] fn test_bitfields_fifth () { let mut date : bindings :: bitfields :: Fifth = unsafe { mem :: zeroed () } ; assert ! (unsafe { date . assert (0 , 0 , 0 , 0 , 0) }) ; date . byte = 255 ; date . set_nWeekDay (6) ; date . set_nMonthDay (20) ; date . set_nMonth (11) ; date . set_nYear (95) ; assert ! (unsafe { date . assert (6 , 20 , 11 , 95 , 255) }) ; }
};
}
