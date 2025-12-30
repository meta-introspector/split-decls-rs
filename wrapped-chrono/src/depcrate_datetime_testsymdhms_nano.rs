// Generated macro for ymdhms_nano (function)
macro_rules! Depcrate_datetime_testsymdhms_nano {
() => {
// Module: crate::datetime::tests
// Provides: {"ymdhms_nano"}
// Dependencies: {}
# [allow (clippy :: too_many_arguments)] # [cfg (feature = "alloc")] fn ymdhms_nano (fixedoffset : & FixedOffset , year : i32 , month : u32 , day : u32 , hour : u32 , min : u32 , sec : u32 , nano : u32 ,) -> DateTime < FixedOffset > { fixedoffset . with_ymd_and_hms (year , month , day , hour , min , sec) . unwrap () . with_nanosecond (nano) . unwrap () }
};
}
