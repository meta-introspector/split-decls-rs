// Generated macro for other_2248 (other)
macro_rules! Depcrate_generatedother_2248 {
() => {
// Module: crate::generated
// Provides: {"other_2248"}
// Dependencies: {}
extern "C-unwind" { # [doc = " Obtain the clock's playback rate."] # [doc = ""] # [doc = " Returns the clock's current play rate. If the clock is internally synced,"] # [doc = " this will be the last rate set by CAClockSetPlayRate. If the clock is"] # [doc = " externally synced, it will be the rate of the external sync source, where"] # [doc = " 1.0 means that it is running at exactly the same rate as the clock's"] # [doc = " timebase. (2.0 means twice as fast)."] # [doc = ""] # [doc = ""] # [doc = " Parameter `inCAClock`: The clock object."] # [doc = ""] # [doc = ""] # [doc = " Parameter `outPlayRate`: On exit, the clock's playback rate."] # [doc = ""] # [doc = ""] # [doc = " Returns: An OSStatus error code."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `in_ca_clock` must be a valid pointer."] # [doc = " - `out_play_rate` must be a valid pointer."] pub fn CAClockGetPlayRate (in_ca_clock : CAClockRef , out_play_rate : NonNull < f64 >) -> OSStatus ; }
};
}
