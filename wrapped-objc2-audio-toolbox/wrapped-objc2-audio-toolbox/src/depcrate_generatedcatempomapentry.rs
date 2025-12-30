// Generated macro for CATempoMapEntry (struct)
macro_rules! Depcrate_generatedCATempoMapEntry {
() => {
// Module: crate::generated
// Provides: {"CATempoMapEntry"}
// Dependencies: {}
# [doc = " A tempo change event."] # [doc = ""] # [doc = " The clock's tempo map defines the correspondence between seconds and musical"] # [doc = " beats, and is used in conversions between the two."] # [doc = ""] # [doc = ""] # [doc = " The beat time at which the tempo changes."] # [doc = ""] # [doc = " The new tempo as of that time."] # [doc = ""] # [doc = " See also [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/catempomapentry?language=objc)"] # [repr (C)] # [derive (Clone , Copy , Debug , PartialEq , Default)] pub struct CATempoMapEntry { pub beats : CAClockBeats , pub tempoBPM : CAClockTempo , }
};
}
