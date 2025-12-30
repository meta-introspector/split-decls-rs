// Generated macro for kAudioFormatFlagsAudioUnitCanonical (const)
macro_rules! Depcrate_base_typeskAudioFormatFlagsAudioUnitCanonical {
() => {
// Module: crate::base_types
// Provides: {"kAudioFormatFlagsAudioUnitCanonical"}
// Dependencies: {}
# [doc = " [Apple's documentation](https://developer.apple.com/documentation/coreaudiotypes/kaudioformatflagsaudiounitcanonical?language=objc)"] # [cfg (not (all (all (target_vendor = "apple" , not (target_os = "macos")) , not (target_env = "macabi") ,)))] # [deprecated] pub const kAudioFormatFlagsAudioUnitCanonical : AudioFormatFlags = kAudioFormatFlagIsSignedInteger | kAudioFormatFlagsNativeEndian | kAudioFormatFlagIsPacked | kAudioFormatFlagIsNonInterleaved | (kAudioUnitSampleFractionBits << kLinearPCMFormatFlagsSampleFractionShift) ;
};
}
