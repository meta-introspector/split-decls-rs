// Generated macro for impl_1236 (impl)
macro_rules! Depcrate_timeimpl_1236 {
() => {
// Module: crate::time
// Provides: {"impl_1236"}
// Dependencies: {}
impl IndexTime { # [doc = " Creates a new time structure from its components."] pub fn new (seconds : i32 , nanoseconds : u32) -> IndexTime { unsafe { Binding :: from_raw (raw :: git_index_time { seconds , nanoseconds , }) } } # [doc = " Returns the number of seconds in the second component of this time."] pub fn seconds (& self) -> i32 { self . raw . seconds } # [doc = " Returns the nanosecond component of this time."] pub fn nanoseconds (& self) -> u32 { self . raw . nanoseconds } }
};
}
