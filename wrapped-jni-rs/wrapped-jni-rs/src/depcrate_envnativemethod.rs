// Generated macro for NativeMethod (struct)
macro_rules! Depcrate_envNativeMethod {
() => {
// Module: crate::env
// Provides: {"NativeMethod"}
// Dependencies: {}
# [derive (Debug)] # [doc = " Native method descriptor."] pub struct NativeMethod { # [doc = " Name of method."] pub name : JNIString , # [doc = " Method signature."] pub sig : JNIString , # [doc = " Pointer to native function with signature"] # [doc = " `fn(env: Env, class: JClass, ...arguments according to sig) -> RetType`"] # [doc = " for static methods or"] # [doc = " `fn(env: Env, object: JObject, ...arguments according to sig) -> RetType`"] # [doc = " for instance methods."] pub fn_ptr : * mut c_void , }
};
}
