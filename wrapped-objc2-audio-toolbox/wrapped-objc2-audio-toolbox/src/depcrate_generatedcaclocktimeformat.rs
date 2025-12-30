// Generated macro for CAClockTimeFormat (struct)
macro_rules! Depcrate_generatedCAClockTimeFormat {
() => {
// Module: crate::generated
// Provides: {"CAClockTimeFormat"}
// Dependencies: {}
# [doc = " The various units in which a clock can represent and report time."] # [doc = ""] # [doc = ""] # [doc = " Absolute host time, as returned by"] # [doc = " <code>"] # [doc = " mach_absolute_time()"] # [doc = " </code>"] # [doc = " ."] # [doc = ""] # [doc = " Absolute audio samples, as a Float64. Available when the internal timebase"] # [doc = " is an audio device (or audio output unit). The units are in arbitrary sample"] # [doc = " numbers, corresponding to the audio device's current time, and at the"] # [doc = " device's current sample rate."] # [doc = ""] # [doc = " Musical beats, as a Float64. This is a position on the clock's timeline."] # [doc = ""] # [doc = " Seconds, as a Float64. This is a position on the clock's timeline."] # [doc = ""] # [doc = " Seconds, as a Float64. This is the same as kCAClockTimeFormat_Seconds,"] # [doc = " except that the clock's SMPTE offset has been applied."] # [doc = ""] # [doc = " SMPTETime structure."] # [doc = ""] # [doc = " See also [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/caclocktimeformat?language=objc)"] # [repr (transparent)] # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash , PartialOrd , Ord)] pub struct CAClockTimeFormat (pub u32) ;
};
}
