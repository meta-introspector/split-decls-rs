// Generated macro for AudioHardwareServiceHasProperty (function)
macro_rules! Depcrate_generatedAudioHardwareServiceHasProperty {
() => {
// Module: crate::generated
// Provides: {"AudioHardwareServiceHasProperty"}
// Dependencies: {}
# [doc = " Queries an AudioObject about whether or not it has the given property."] # [doc = ""] # [doc = " Parameter `inObjectID`: The AudioObject to query."] # [doc = ""] # [doc = " Parameter `inAddress`: An AudioObjectPropertyAddress indicating which property is being queried."] # [doc = ""] # [doc = " Returns: A Boolean indicating whether or not the AudioObject has the given property."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `in_address` must be a valid pointer."] # [cfg (feature = "objc2-core-audio")] # [deprecated = "no longer supported"] # [inline] pub unsafe extern "C-unwind" fn AudioHardwareServiceHasProperty (in_object_id : AudioObjectID , in_address : * const AudioObjectPropertyAddress ,) -> bool { extern "C-unwind" { fn AudioHardwareServiceHasProperty (in_object_id : AudioObjectID , in_address : * const AudioObjectPropertyAddress ,) -> Boolean ; } let ret = unsafe { AudioHardwareServiceHasProperty (in_object_id , in_address) } ; ret != 0 }
};
}
