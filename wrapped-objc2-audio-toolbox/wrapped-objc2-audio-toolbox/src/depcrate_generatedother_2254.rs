// Generated macro for other_2254 (other)
macro_rules! Depcrate_generatedother_2254 {
() => {
// Module: crate::generated
// Provides: {"other_2254"}
// Dependencies: {}
extern "C-unwind" { # [doc = " This will return the name of a sound bank from a DLS or SF2 bank."] # [doc = " The name should be released by the caller."] # [doc = ""] # [doc = ""] # [doc = " Parameter `inURL`: The URL for the sound bank."] # [doc = ""] # [doc = " Parameter `outName`: A pointer to a CFStringRef to be created and returned by the function."] # [doc = ""] # [doc = " Returns: returns noErr if successful."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `out_name` must be a valid pointer."] # [cfg (feature = "objc2-core-foundation")] pub fn CopyNameFromSoundBank (in_url : & CFURL , out_name : NonNull < * const CFString >) -> OSStatus ; }
};
}
