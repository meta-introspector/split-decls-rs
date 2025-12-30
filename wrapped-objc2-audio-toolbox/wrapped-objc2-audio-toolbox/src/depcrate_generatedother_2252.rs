// Generated macro for other_2252 (other)
macro_rules! Depcrate_generatedother_2252 {
() => {
// Module: crate::generated
// Provides: {"other_2252"}
// Dependencies: {}
extern "C-unwind" { # [doc = " Converts a CABarBeatTime structure to a number of beats."] # [doc = ""] # [doc = " Converts a CABarBeatTime structure (bars/beats/subbeats) to a beat"] # [doc = " position, using the clock's meter track."] # [doc = ""] # [doc = ""] # [doc = " Parameter `inCAClock`: The clock object."] # [doc = ""] # [doc = ""] # [doc = " Parameter `inBarBeatTime`: The bar/beat/subunit time to be converted to beats."] # [doc = ""] # [doc = ""] # [doc = " Parameter `outBeats`: On exit, the number of absolute beats corresponding to inBarBeatTime."] # [doc = ""] # [doc = ""] # [doc = " Returns: An OSStatus error code."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `in_ca_clock` must be a valid pointer."] # [doc = " - `in_bar_beat_time` must be a valid pointer."] # [doc = " - `out_beats` must be a valid pointer."] # [cfg (feature = "MusicPlayer")] pub fn CAClockBarBeatTimeToBeats (in_ca_clock : CAClockRef , in_bar_beat_time : NonNull < CABarBeatTime > , out_beats : NonNull < CAClockBeats > ,) -> OSStatus ; }
};
}
