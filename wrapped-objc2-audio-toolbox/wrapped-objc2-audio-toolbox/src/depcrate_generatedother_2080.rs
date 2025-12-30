// Generated macro for other_2080 (other)
macro_rules! Depcrate_generatedother_2080 {
() => {
// Module: crate::generated
// Provides: {"other_2080"}
// Dependencies: {}
extern "C-unwind" { # [doc = " Open an existing audio file."] # [doc = ""] # [doc = " Open an existing audio file for reading or reading and writing."] # [doc = ""] # [doc = " Parameter `inComponent`: an AudioFileComponent."] # [doc = ""] # [doc = " Parameter `inFileRef`: the CFURLRef of an existing audio file."] # [doc = ""] # [doc = " Parameter `inPermissions`: use the permission constants."] # [doc = ""] # [doc = " Parameter `inFileDescriptor`: an open file descriptor."] # [doc = ""] # [doc = " Returns: returns noErr if successful."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `in_component` must be a valid pointer."] # [cfg (all (feature = "AudioComponent" , feature = "objc2-core-foundation"))] pub fn AudioFileComponentOpenURL (in_component : AudioFileComponent , in_file_ref : & CFURL , in_permissions : i8 , in_file_descriptor : c_int ,) -> OSStatus ; }
};
}
