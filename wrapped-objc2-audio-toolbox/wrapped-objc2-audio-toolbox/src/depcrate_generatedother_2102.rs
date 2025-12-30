// Generated macro for other_2102 (other)
macro_rules! Depcrate_generatedother_2102 {
() => {
// Module: crate::generated
// Provides: {"other_2102"}
// Dependencies: {}
extern "C-unwind" { # [doc = " deprecated. use AudioFileComponentFileDataIsThisFormat instead."] # [doc = ""] # [doc = " Parameter `inComponent`: an AudioFileComponent"] # [doc = ""] # [doc = " Parameter `inFileRefNum`: a refNum of a file."] # [doc = ""] # [doc = " Parameter `outResult`: on output, is set to 1 if the file is recognized by this component, 0 if not."] # [doc = ""] # [doc = " Returns: returns noErr if successful."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `in_component` must be a valid pointer."] # [doc = " - `out_result` must be a valid pointer."] # [cfg (feature = "AudioComponent")] # [deprecated = "no longer supported"] pub fn AudioFileComponentFileIsThisFormat (in_component : AudioFileComponent , in_file_ref_num : i16 , out_result : NonNull < u32 > ,) -> OSStatus ; }
};
}
