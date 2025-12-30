// Generated macro for other_2251 (other)
macro_rules! Depcrate_generatedother_2251 {
() => {
// Module: crate::generated
// Provides: {"other_2251"}
// Dependencies: {}
extern "C-unwind" { # [doc = " Converts a number of beats to a CABarBeatTime structure."] # [doc = ""] # [doc = " Converts a beat position on the media timeline to a CABarBeatTime, using the"] # [doc = " clock's meter track. Examples using 4/4 time and a subbeat divisor of 480:"] # [doc = ""] # [doc = " inBeats | outBarBeatTime: bars . beats . units"] # [doc = " --------|-------------------------------------"] # [doc = " 0        | 1.1.0"] # [doc = " 1        | 1.2.0"] # [doc = " 4        | 2.1.0"] # [doc = " 4.5        | 2.1.240"] # [doc = ""] # [doc = ""] # [doc = " Parameter `inCAClock`: The clock object."] # [doc = ""] # [doc = ""] # [doc = " Parameter `inBeats`: The absolute beat count to be converted."] # [doc = ""] # [doc = ""] # [doc = " Parameter `inSubbeatDivisor`: The number of units per beat."] # [doc = ""] # [doc = ""] # [doc = " Parameter `outBarBeatTime`: On exit, the bar/beat/subbeat time corresponding to inBeats."] # [doc = ""] # [doc = ""] # [doc = " Returns: An OSStatus error code."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `in_ca_clock` must be a valid pointer."] # [doc = " - `out_bar_beat_time` must be a valid pointer."] # [cfg (feature = "MusicPlayer")] pub fn CAClockBeatsToBarBeatTime (in_ca_clock : CAClockRef , in_beats : CAClockBeats , in_subbeat_divisor : u16 , out_bar_beat_time : NonNull < CABarBeatTime > ,) -> OSStatus ; }
};
}
