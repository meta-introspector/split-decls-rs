// Generated macro for other_2083 (other)
macro_rules! Depcrate_generatedother_2083 {
() => {
// Module: crate::generated
// Provides: {"other_2083"}
// Dependencies: {}
extern "C-unwind" { # [doc = " implements AudioFileClose."] # [doc = ""] # [doc = " Parameter `inComponent`: an AudioFileComponent"] # [doc = ""] # [doc = " Returns: returns noErr if successful."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `in_component` must be a valid pointer."] # [cfg (feature = "AudioComponent")] pub fn AudioFileComponentCloseFile (in_component : AudioFileComponent) -> OSStatus ; }
};
}
