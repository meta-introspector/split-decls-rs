// Generated macro for JNIInvokeInterface_ (struct)
macro_rules! DepcrateJNIInvokeInterface_ {
() => {
// Module: crate
// Provides: {"JNIInvokeInterface_"}
// Dependencies: {}
# [repr (C)] # [jni_to_union] # [non_exhaustive] # [derive (Copy , Clone)] pub struct JNIInvokeInterface_ { # [jni_added ("reserved")] pub reserved0 : * mut c_void , # [jni_added ("reserved")] pub reserved1 : * mut c_void , # [jni_added ("reserved")] pub reserved2 : * mut c_void , pub DestroyJavaVM : unsafe extern "system" fn (vm : * mut JavaVM) -> jint , pub AttachCurrentThread : unsafe extern "system" fn (vm : * mut JavaVM , penv : * mut * mut c_void , args : * mut c_void ,) -> jint , pub DetachCurrentThread : unsafe extern "system" fn (vm : * mut JavaVM) -> jint , # [jni_added ("1.2")] pub GetEnv : unsafe extern "system" fn (vm : * mut JavaVM , penv : * mut * mut c_void , version : jint) -> jint , # [jni_added ("1.4")] pub AttachCurrentThreadAsDaemon : unsafe extern "system" fn (vm : * mut JavaVM , penv : * mut * mut c_void , args : * mut c_void ,) -> jint , }
};
}
