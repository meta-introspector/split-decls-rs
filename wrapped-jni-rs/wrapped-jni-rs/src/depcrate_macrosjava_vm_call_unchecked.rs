// Generated macro for java_vm_call_unchecked (macro)
macro_rules! Depcrate_macrosjava_vm_call_unchecked {
() => {
// Module: crate::macros
// Provides: {"java_vm_call_unchecked"}
// Dependencies: {}
# [doc = " Directly calls a JavaVM function, nothing else"] macro_rules ! java_vm_call_unchecked { ($ jvm : expr , $ version : tt , $ name : tt $ (, $ args : expr) *) => { { let jvm : * mut jni_sys :: JavaVM = $ jvm . get_raw () ; ((* (* jvm)) .$ version .$ name) (jvm $ (, $ args) *) } } ; }
};
}
