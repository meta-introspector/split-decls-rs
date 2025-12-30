// Generated macro for ymdhms_utc (function)
macro_rules! Depcrate_datetime_testsymdhms_utc {
() => {
// Module: crate::datetime::tests
// Provides: {"ymdhms_utc"}
// Dependencies: {}
# [cfg (feature = "alloc")] fn ymdhms_utc (year : i32 , month : u32 , day : u32 , hour : u32 , min : u32 , sec : u32) -> DateTime < Utc > { Utc . with_ymd_and_hms (year , month , day , hour , min , sec) . unwrap () }
};
}
