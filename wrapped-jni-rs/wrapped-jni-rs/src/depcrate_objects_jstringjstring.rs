// Generated macro for JString (struct)
macro_rules! Depcrate_objects_jstringJString {
() => {
// Module: crate::objects::jstring
// Provides: {"JString"}
// Dependencies: {}
# [doc = " A `java.lang.String` wrapper that is tied to a JNI local reference frame."] # [doc = ""] # [doc = " See the [`JObject`] documentation for more information about reference"] # [doc = " wrappers, how to cast them, and local reference frame lifetimes."] # [doc = ""] # [repr (transparent)] # [derive (Debug , Default)] pub struct JString < 'local > (JObject < 'local >) ;
};
}
