// Generated macro for JClass (struct)
macro_rules! Depcrate_objects_jclassJClass {
() => {
// Module: crate::objects::jclass
// Provides: {"JClass"}
// Dependencies: {}
# [doc = " A `java.lang.Class` wrapper that is tied to a JNI local reference frame."] # [doc = ""] # [doc = " See the [`JObject`] documentation for more information about reference"] # [doc = " wrappers, how to cast them, and local reference frame lifetimes."] # [doc = ""] # [repr (transparent)] # [derive (Debug , Default)] pub struct JClass < 'local > (JObject < 'local >) ;
};
}
