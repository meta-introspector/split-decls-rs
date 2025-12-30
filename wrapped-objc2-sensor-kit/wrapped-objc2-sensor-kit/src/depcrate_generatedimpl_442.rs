// Generated macro for impl_442 (impl)
macro_rules! Depcrate_generatedimpl_442 {
() => {
// Module: crate::generated
// Provides: {"impl_442"}
// Dependencies: {}
impl SRAcousticSettingsAccessibility { extern_methods ! (# [doc = " Audio volume between left and right channels"] # [doc = ""] # [doc = " This property is not atomic."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This might not be thread-safe."] # [unsafe (method (leftRightBalance))] # [unsafe (method_family = none)] pub unsafe fn leftRightBalance (& self) -> c_double ; # [doc = " When in mono mode, audio output is the same audio from both the left and right channels"] # [doc = ""] # [doc = " This property is not atomic."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This might not be thread-safe."] # [unsafe (method (isMonoAudioEnabled))] # [unsafe (method_family = none)] pub unsafe fn isMonoAudioEnabled (& self) -> bool ; # [doc = " Background Sounds Settings"] # [doc = ""] # [doc = " This property is not atomic."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This might not be thread-safe."] # [unsafe (method (backgroundSounds))] # [unsafe (method_family = none)] pub unsafe fn backgroundSounds (& self ,) -> Retained < SRAcousticSettingsAccessibilityBackgroundSounds >; # [doc = " Headphone Accommodations Settings"] # [doc = ""] # [doc = " This property is not atomic."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This might not be thread-safe."] # [unsafe (method (headphoneAccommodations))] # [unsafe (method_family = none)] pub unsafe fn headphoneAccommodations (& self ,) -> Retained < SRAcousticSettingsAccessibilityHeadphoneAccommodations >;) ; }
};
}
