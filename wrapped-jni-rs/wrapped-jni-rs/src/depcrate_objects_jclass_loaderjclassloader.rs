// Generated macro for JClassLoader (struct)
macro_rules! Depcrate_objects_jclass_loaderJClassLoader {
() => {
// Module: crate::objects::jclass_loader
// Provides: {"JClassLoader"}
// Dependencies: {}
# [doc = " A `java.lang.ClassLoader` wrapper that is tied to a JNI local reference frame."] # [doc = ""] # [doc = " See the [`JObject`] documentation for more information about reference"] # [doc = " wrappers, how to cast them, and local reference frame lifetimes."] # [doc = ""] # [repr (transparent)] # [derive (Debug , Default)] pub struct JClassLoader < 'local > (JObject < 'local >) ;
};
}
