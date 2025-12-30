// Generated macro for CAMeterTrackEntry (struct)
macro_rules! Depcrate_generatedCAMeterTrackEntry {
() => {
// Module: crate::generated
// Provides: {"CAMeterTrackEntry"}
// Dependencies: {}
# [doc = " A time signature change event."] # [doc = ""] # [doc = " The meter track is used for converting between beats as floating-point"] # [doc = " numbers (CAClockBeats) and their display representations (CABarBeatTime)."] # [doc = ""] # [doc = ""] # [doc = " The beat time at which the time signature (meter) changes."] # [doc = ""] # [doc = " The numerator of the new time signature."] # [doc = ""] # [doc = " The denominator of the new time signature (1, 2, 4, 8, etc.)."] # [doc = ""] # [doc = " See also [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/cametertrackentry?language=objc)"] # [repr (C)] # [derive (Clone , Copy , Debug , PartialEq , Default)] pub struct CAMeterTrackEntry { pub beats : CAClockBeats , pub meterNumer : u16 , pub meterDenom : u16 , }
};
}
