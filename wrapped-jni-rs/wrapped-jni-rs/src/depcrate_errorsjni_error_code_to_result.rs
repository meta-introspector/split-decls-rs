// Generated macro for jni_error_code_to_result (function)
macro_rules! Depcrate_errorsjni_error_code_to_result {
() => {
// Module: crate::errors
// Provides: {"jni_error_code_to_result"}
// Dependencies: {}
pub fn jni_error_code_to_result (code : sys :: jint) -> Result < () > { match code { sys :: JNI_OK => Ok (()) , sys :: JNI_ERR => Err (JniError :: Unknown) , sys :: JNI_EDETACHED => Err (JniError :: ThreadDetached) , sys :: JNI_EVERSION => Err (JniError :: WrongVersion) , sys :: JNI_ENOMEM => Err (JniError :: NoMemory) , sys :: JNI_EEXIST => Err (JniError :: AlreadyCreated) , sys :: JNI_EINVAL => Err (JniError :: InvalidArguments) , _ => Err (JniError :: Other (code)) , } . map_err (Error :: JniCall) }
};
}
