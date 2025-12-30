// Generated macro for other_2079 (other)
macro_rules! Depcrate_generatedother_2079 {
() => {
// Module: crate::generated
// Provides: {"other_2079"}
// Dependencies: {}
extern "C-unwind" { # [doc = " creates a new (or initialises an existing) audio file specified by the URL."] # [doc = ""] # [doc = " creates a new (or initialises an existing) audio file specified by the URL."] # [doc = ""] # [doc = " Parameter `inComponent`: an AudioFileComponent"] # [doc = ""] # [doc = " Parameter `inFileRef`: an CFURLRef fully specifying the path of the file to create/initialise"] # [doc = ""] # [doc = " Parameter `inFormat`: an AudioStreamBasicDescription describing the data format that will be"] # [doc = " added to the audio file."] # [doc = ""] # [doc = " Parameter `inFlags`: relevant flags for creating/opening the file."] # [doc = " if kAudioFileFlags_EraseFile is set, it will erase an existing file"] # [doc = " if not set, then the Create call will fail if the URL is an existing file"] # [doc = ""] # [doc = " Returns: returns noErr if successful."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `in_component` must be a valid pointer."] # [doc = " - `in_format` must be a valid pointer."] # [cfg (all (feature = "AudioComponent" , feature = "objc2-core-audio-types" , feature = "objc2-core-foundation"))] pub fn AudioFileComponentCreateURL (in_component : AudioFileComponent , in_file_ref : & CFURL , in_format : NonNull < AudioStreamBasicDescription > , in_flags : u32 ,) -> OSStatus ; }
};
}
