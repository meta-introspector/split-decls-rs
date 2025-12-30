// Generated macro for ymdhms_milli_utc (function)
macro_rules! Depcrate_datetime_testsymdhms_milli_utc {
() => {
// Module: crate::datetime::tests
// Provides: {"ymdhms_milli_utc"}
// Dependencies: {}
fn ymdhms_milli_utc (year : i32 , month : u32 , day : u32 , hour : u32 , min : u32 , sec : u32 , milli : u32 ,) -> DateTime < Utc > { Utc . with_ymd_and_hms (year , month , day , hour , min , sec) . unwrap () . with_nanosecond (milli * 1_000_000) . unwrap () }
};
}
