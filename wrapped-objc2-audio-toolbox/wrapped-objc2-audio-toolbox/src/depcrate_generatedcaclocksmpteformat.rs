// Generated macro for CAClockSMPTEFormat (type)
macro_rules! Depcrate_generatedCAClockSMPTEFormat {
() => {
// Module: crate::generated
// Provides: {"CAClockSMPTEFormat"}
// Dependencies: {}
# [doc = " A SMPTE format, specifying the frames per second (fps) and"] # [doc = " whether it is drop frame."] # [doc = ""] # [doc = " The possible values of a CAClockSMPTEFormat are found in"] # [doc = " <CoreAudioTypes"] # [doc = " /CoreAudioTypes.h>."] # [doc = " Values include kSMPTETimeType30, kSMPTETimeType30Drop, etc. Note that formats with more than 30"] # [doc = " fps are not usable with MIDI Time Code."] # [doc = ""] # [doc = " See also [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/caclocksmpteformat?language=objc)"] # [cfg (feature = "objc2-core-audio-types")] pub type CAClockSMPTEFormat = SMPTETimeType ;
};
}
