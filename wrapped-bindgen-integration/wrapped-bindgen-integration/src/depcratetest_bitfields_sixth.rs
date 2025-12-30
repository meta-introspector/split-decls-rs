// Generated macro for test_bitfields_sixth (function)
macro_rules! Depcratetest_bitfields_sixth {
() => {
// Module: crate
// Provides: {"test_bitfields_sixth"}
// Dependencies: {}
# [test] fn test_bitfields_sixth () { let mut date : bindings :: bitfields :: Sixth = unsafe { mem :: zeroed () } ; assert ! (unsafe { date . assert (0 , 0 , 0 , 0) }) ; date . byte = 255 ; date . set_nWeekDay (6) ; date . set_nMonthDay (20) ; date . set_nMonth (11) ; assert ! (unsafe { date . assert (255 , 6 , 11 , 20) }) ; }
};
}
