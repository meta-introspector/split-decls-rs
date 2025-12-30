// Generated macro for other_2101 (other)
macro_rules! Depcrate_generatedother_2101 {
() => {
// Module: crate::generated
// Provides: {"other_2101"}
// Dependencies: {}
extern "C-unwind" { # [doc = " used by the AudioFile API to determine if this component is appropriate for handling a file."] # [doc = ""] # [doc = " Parameter `inComponent`: an AudioFileComponent"] # [doc = ""] # [doc = " Parameter `inDataByteSize`: the size of inData in bytes."] # [doc = ""] # [doc = " Parameter `inData`: a pointer to a buffer of audio file data."] # [doc = ""] # [doc = " Parameter `outResult`: on output, is set to 1 if the file is recognized by this component, 0 if not."] # [doc = ""] # [doc = " Returns: returns noErr if successful."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `in_component` must be a valid pointer."] # [doc = " - `in_data` must be a valid pointer."] # [doc = " - `out_result` must be a valid pointer."] # [cfg (feature = "AudioComponent")] pub fn AudioFileComponentFileDataIsThisFormat (in_component : AudioFileComponent , in_data_byte_size : u32 , in_data : NonNull < c_void > , out_result : NonNull < u32 > ,) -> OSStatus ; }
};
}
