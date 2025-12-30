// Generated macro for test_bitfields_date2 (function)
macro_rules! Depcratetest_bitfields_date2 {
() => {
// Module: crate
// Provides: {"test_bitfields_date2"}
// Dependencies: {}
# [test] fn test_bitfields_date2 () { let mut date : bindings :: bitfields :: Date2 = unsafe { mem :: zeroed () } ; assert ! (unsafe { date . assert (0 , 0 , 0 , 0 , 0) }) ; date . set_nWeekDay (6) ; date . set_nMonthDay (20) ; date . set_nMonth (11) ; date . set_nYear (95) ; date . set_byte (255) ; assert ! (unsafe { date . assert (6 , 20 , 11 , 95 , 255) }) ; }
};
}
