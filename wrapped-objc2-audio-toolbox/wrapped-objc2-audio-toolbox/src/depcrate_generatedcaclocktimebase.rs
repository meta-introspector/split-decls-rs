// Generated macro for CAClockTimebase (struct)
macro_rules! Depcrate_generatedCAClockTimebase {
() => {
// Module: crate::generated
// Provides: {"CAClockTimebase"}
// Dependencies: {}
# [doc = " The available internal hardware time references for a clock."] # [doc = ""] # [doc = ""] # [doc = " The clock's reference time is host time (as returned"] # [doc = " by"] # [doc = " <code>"] # [doc = " mach_absolute_time()"] # [doc = " </code>"] # [doc = " or"] # [doc = " <code>"] # [doc = " HostTime()"] # [doc = " </code>"] # [doc = " )."] # [doc = ""] # [doc = " The clock's reference time is derived from an audio"] # [doc = " device."] # [doc = ""] # [doc = " The clock's reference time is derived from the audio"] # [doc = " device addressed by an output Audio Unit."] # [doc = ""] # [doc = " See also [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/caclocktimebase?language=objc)"] # [repr (transparent)] # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash , PartialOrd , Ord)] pub struct CAClockTimebase (pub u32) ;
};
}
