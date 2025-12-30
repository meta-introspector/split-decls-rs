// Generated macro for jni_call_unchecked (macro)
macro_rules! Depcrate_macrosjni_call_unchecked {
() => {
// Module: crate::macros
// Provides: {"jni_call_unchecked"}
// Dependencies: {}
# [doc = " Directly calls a Env FFI function, nothing else"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " When calling any function added after JNI 1.1 you must know that it's valid"] # [doc = " for the current JNI version."] macro_rules ! jni_call_unchecked { ($ jnienv : expr , $ version : tt , $ name : tt $ (, $ args : expr) *) => { { let env : * mut jni_sys :: JNIEnv = $ jnienv . get_raw () ; let interface : * const jni_sys :: JNINativeInterface_ = * env ; ((* interface) .$ version .$ name) (env $ (, $ args) *) } } ; }
};
}
