// Generated macro for jni_call_check_ex (macro)
macro_rules! Depcrate_macrosjni_call_check_ex {
() => {
// Module: crate::macros
// Provides: {"jni_call_check_ex"}
// Dependencies: {}
# [doc = " Calls a Env function, then checks for a pending exception"] # [doc = ""] # [doc = " This only checks for an exception, it doesn't clear the exception and so the"] # [doc = " exception will be thrown if the native code returns to the JVM."] # [doc = ""] # [doc = " Returns `Err` if there is a pending exception after the call."] macro_rules ! jni_call_check_ex { ($ jnienv : expr , $ version : tt , $ name : tt $ (, $ args : expr) *) => ({ let ret = jni_call_unchecked ! ($ jnienv , $ version , $ name $ (, $ args) *) ; if $ jnienv . exception_check () { Err (crate :: errors :: Error :: JavaException) } else { Ok (ret) } }) }
};
}
