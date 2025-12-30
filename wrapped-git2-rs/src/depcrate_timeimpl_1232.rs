// Generated macro for impl_1232 (impl)
macro_rules! Depcrate_timeimpl_1232 {
() => {
// Module: crate::time
// Provides: {"impl_1232"}
// Dependencies: {}
impl Time { # [doc = " Creates a new time structure from its components."] pub fn new (time : i64 , offset : i32) -> Time { unsafe { Binding :: from_raw (raw :: git_time { time : time as raw :: git_time_t , offset : offset as c_int , sign : if offset < 0 { '-' } else { '+' } as c_char , }) } } # [doc = " Return the time, in seconds, from epoch"] pub fn seconds (& self) -> i64 { self . raw . time as i64 } # [doc = " Return the timezone offset, in minutes"] pub fn offset_minutes (& self) -> i32 { self . raw . offset as i32 } # [doc = " Return whether the offset was positive or negative. Primarily useful"] # [doc = " in case the offset is specified as a negative zero."] pub fn sign (& self) -> char { self . raw . sign as u8 as char } }
};
}
