// Generated macro for other_2247 (other)
macro_rules! Depcrate_generatedother_2247 {
() => {
// Module: crate::generated
// Provides: {"other_2247"}
// Dependencies: {}
extern "C-unwind" { # [doc = " Alter the clock's playback rate."] # [doc = ""] # [doc = " Adjusts the ratio between the timebase and media time; e.g. at 0.5, the"] # [doc = " media time will move half as quickly as timebase time."] # [doc = ""] # [doc = ""] # [doc = " Parameter `inCAClock`: The clock object."] # [doc = ""] # [doc = ""] # [doc = " Parameter `inPlayRate`: The clock's desired play rate."] # [doc = ""] # [doc = ""] # [doc = " Returns: An OSStatus error code."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `in_ca_clock` must be a valid pointer."] pub fn CAClockSetPlayRate (in_ca_clock : CAClockRef , in_play_rate : f64) -> OSStatus ; }
};
}
