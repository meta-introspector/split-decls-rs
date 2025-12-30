// Generated macro for other_2100 (other)
macro_rules! Depcrate_generatedother_2100 {
() => {
// Module: crate::generated
// Provides: {"other_2100"}
// Dependencies: {}
extern "C-unwind" { # [doc = " used by the AudioFile API to determine if this component is appropriate for handling a file."] # [doc = ""] # [doc = " Parameter `inComponent`: an AudioFileComponent"] # [doc = ""] # [doc = " Parameter `inExtension`: a CFString containing a file name extension."] # [doc = ""] # [doc = " Parameter `outResult`: on output, is set to 1 if the extension is recognized by this component, 0 if not."] # [doc = ""] # [doc = " Returns: returns noErr if successful."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `in_component` must be a valid pointer."] # [doc = " - `out_result` must be a valid pointer."] # [cfg (all (feature = "AudioComponent" , feature = "objc2-core-foundation"))] pub fn AudioFileComponentExtensionIsThisFormat (in_component : AudioFileComponent , in_extension : & CFString , out_result : NonNull < u32 > ,) -> OSStatus ; }
};
}
