// Generated macro for other_2250 (other)
macro_rules! Depcrate_generatedother_2250 {
() => {
// Module: crate::generated
// Provides: {"other_2250"}
// Dependencies: {}
extern "C-unwind" { # [doc = " Converts a SMPTE time representation to seconds."] # [doc = ""] # [doc = " Converts SMPTE time to seconds on the media timeline. The clock's current"] # [doc = " SMPTE format and offset must be set appropriately."] # [doc = ""] # [doc = ""] # [doc = " Parameter `inCAClock`: The clock object."] # [doc = ""] # [doc = ""] # [doc = " Parameter `inSMPTETime`: The SMPTE time to be converted to seconds."] # [doc = ""] # [doc = ""] # [doc = " Parameter `outSeconds`: On exit, the number of seconds corresponding to inSMPTETime."] # [doc = ""] # [doc = ""] # [doc = " Returns: An OSStatus error code."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `in_ca_clock` must be a valid pointer."] # [doc = " - `in_smpte_time` must be a valid pointer."] # [doc = " - `out_seconds` must be a valid pointer."] # [cfg (feature = "objc2-core-audio-types")] pub fn CAClockSMPTETimeToSeconds (in_ca_clock : CAClockRef , in_smpte_time : NonNull < SMPTETime > , out_seconds : NonNull < CAClockSeconds > ,) -> OSStatus ; }
};
}
