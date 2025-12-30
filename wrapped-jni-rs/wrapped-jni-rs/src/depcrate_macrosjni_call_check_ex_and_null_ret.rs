// Generated macro for jni_call_check_ex_and_null_ret (macro)
macro_rules! Depcrate_macrosjni_call_check_ex_and_null_ret {
() => {
// Module: crate::macros
// Provides: {"jni_call_check_ex_and_null_ret"}
// Dependencies: {}
# [doc = " Calls a Env function, then checks for a pending exception, then checks for a `null` return value"] # [doc = ""] # [doc = " Returns `Err` if there is a pending exception after the call."] # [doc = " Returns `Err(Error::NullPtr)` if the JNI function returns `null`"] macro_rules ! jni_call_check_ex_and_null_ret { ($ jnienv : expr , $ version : tt , $ name : tt $ (, $ args : expr) *) => ({ jni_call_check_ex ! ($ jnienv , $ version , $ name $ (, $ args) *) . and_then (| ret | { if ret . is_null () { Err ($ crate :: errors :: Error :: NullPtr (concat ! (stringify ! ($ name) , " result"))) } else { Ok (ret) } }) }) }
};
}
