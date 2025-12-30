// Generated macro for other_2071 (other)
macro_rules! Depcrate_generatedother_2071 {
() => {
// Module: crate::generated
// Provides: {"other_2071"}
// Dependencies: {}
extern "C-unwind" { # [doc = " Queries an AudioObject about whether or not the given property can be set using"] # [doc = " AudioHardwareServiceSetPropertyData."] # [doc = ""] # [doc = " Parameter `inObjectID`: The AudioObject to query."] # [doc = ""] # [doc = " Parameter `inAddress`: An AudioObjectPropertyAddress indicating which property is being queried."] # [doc = ""] # [doc = " Parameter `outIsSettable`: A Boolean indicating whether or not the property can be set."] # [doc = ""] # [doc = " Returns: An OSStatus indicating success or failure."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `in_address` must be a valid pointer."] # [doc = " - `out_is_settable` must be a valid pointer."] # [cfg (feature = "objc2-core-audio")] # [deprecated = "no longer supported"] pub fn AudioHardwareServiceIsPropertySettable (in_object_id : AudioObjectID , in_address : * const AudioObjectPropertyAddress , out_is_settable : * mut Boolean ,) -> OSStatus ; }
};
}
