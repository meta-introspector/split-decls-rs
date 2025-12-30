// Generated macro for jni_call_only_check_null_ret (macro)
macro_rules! Depcrate_macrosjni_call_only_check_null_ret {
() => {
// Module: crate::macros
// Provides: {"jni_call_only_check_null_ret"}
// Dependencies: {}
# [doc = " Calls a Env function, with no check for exceptions, then checks for a `null` return value"] # [doc = ""] # [doc = " Returns `Err(Error::NullPtr)` if the JNI function returns `null`"] macro_rules ! jni_call_only_check_null_ret { ($ jnienv : expr , $ version : tt , $ name : tt $ (, $ args : expr) *) => ({ let ret = jni_call_unchecked ! ($ jnienv , $ version , $ name $ (, $ args) *) ; if ret . is_null () { Err ($ crate :: errors :: Error :: NullPtr (concat ! (stringify ! ($ name) , " result"))) } else { Ok (ret) } }) }
};
}
