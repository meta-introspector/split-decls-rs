// Generated macro for other_2093 (other)
macro_rules! Depcrate_generatedother_2093 {
() => {
// Module: crate::generated
// Provides: {"other_2093"}
// Dependencies: {}
extern "C-unwind" { # [doc = " implements AudioFileCountUserData"] # [doc = ""] # [doc = " \"User Data\" refers to chunks in AIFF, CAF and WAVE files, or resources"] # [doc = " in Sound Designer II files, and possibly other things in other files."] # [doc = " For simplicity, referred to below as \"chunks\"."] # [doc = ""] # [doc = " Parameter `inComponent`: an AudioFileComponent"] # [doc = ""] # [doc = " Parameter `inUserDataID`: the four char code of the chunk."] # [doc = ""] # [doc = " Parameter `outNumberItems`: on output, if successful, number of chunks of this type in the file."] # [doc = ""] # [doc = " Returns: returns noErr if successful."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `in_component` must be a valid pointer."] # [doc = " - `out_number_items` must be a valid pointer."] # [cfg (feature = "AudioComponent")] pub fn AudioFileComponentCountUserData (in_component : AudioFileComponent , in_user_data_id : u32 , out_number_items : NonNull < u32 > ,) -> OSStatus ; }
};
}
