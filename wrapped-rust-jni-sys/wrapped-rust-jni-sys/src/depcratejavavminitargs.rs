// Generated macro for JavaVMInitArgs (struct)
macro_rules! DepcrateJavaVMInitArgs {
() => {
// Module: crate
// Provides: {"JavaVMInitArgs"}
// Dependencies: {}
# [repr (C)] # [derive (Copy , Debug)] pub struct JavaVMInitArgs { pub version : jint , pub nOptions : jint , pub options : * mut JavaVMOption , pub ignoreUnrecognized : jboolean , }
};
}
