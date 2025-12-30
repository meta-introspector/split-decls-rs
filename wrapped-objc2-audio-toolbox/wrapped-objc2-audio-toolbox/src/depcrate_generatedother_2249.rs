// Generated macro for other_2249 (other)
macro_rules! Depcrate_generatedother_2249 {
() => {
// Module: crate::generated
// Provides: {"other_2249"}
// Dependencies: {}
extern "C-unwind" { # [doc = " Converts seconds to a SMPTE time representation."] # [doc = ""] # [doc = " Converts seconds on the media timeline to a SMPTE time. The clock's current"] # [doc = " SMPTE format and offset must be set appropriately."] # [doc = ""] # [doc = ""] # [doc = " Parameter `inCAClock`: The clock object."] # [doc = ""] # [doc = ""] # [doc = " Parameter `inSeconds`: The number of seconds to be converted (e.g. 3600 = 1 hour)."] # [doc = ""] # [doc = ""] # [doc = " Parameter `inSubframeDivisor`: The number of subframes per frame desired in outSMPTETime."] # [doc = ""] # [doc = ""] # [doc = " Parameter `outSMPTETime`: On exit, the SMPTE time corresponding to inSeconds."] # [doc = ""] # [doc = ""] # [doc = " Returns: An OSStatus error code."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `in_ca_clock` must be a valid pointer."] # [doc = " - `out_smpte_time` must be a valid pointer."] # [cfg (feature = "objc2-core-audio-types")] pub fn CAClockSecondsToSMPTETime (in_ca_clock : CAClockRef , in_seconds : CAClockSeconds , in_subframe_divisor : u16 , out_smpte_time : NonNull < SMPTETime > ,) -> OSStatus ; }
};
}
