// Generated macro for other_73 (other)
macro_rules! Depcrateother_73 {
() => {
// Module: crate
// Provides: {"other_73"}
// Dependencies: {}
extern "system" { pub fn JNI_GetDefaultJavaVMInitArgs (args : * mut c_void) -> jint ; pub fn JNI_CreateJavaVM (pvm : * mut * mut JavaVM , penv : * mut * mut c_void , args : * mut c_void ,) -> jint ; pub fn JNI_GetCreatedJavaVMs (vmBuf : * mut * mut JavaVM , bufLen : jsize , nVMs : * mut jsize) -> jint ; }
};
}
