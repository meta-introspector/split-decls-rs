// Generated macro for sys_attach_current_thread (function)
macro_rules! Depcrate_vm_java_vmsys_attach_current_thread {
() => {
// Module: crate::vm::java_vm
// Provides: {"sys_attach_current_thread"}
// Dependencies: {}
unsafe fn sys_attach_current_thread (vm : & JavaVM , config : & AttachConfig , thread : & Thread ,) -> Result < * mut sys :: JNIEnv > { let mut env_ptr = ptr :: null_mut () ; let mut args = sys :: JavaVMAttachArgs { version : JNIVersion :: V1_4 . into () , name : config . name . as_ref () . map (| s | s . as_ptr () as * mut c_char) . unwrap_or (ptr :: null_mut ()) , group : config . group . as_ref () . map (| g | g . as_raw ()) . unwrap_or (ptr :: null_mut ()) , } ; let res = java_vm_call_unchecked ! (vm , v1_1 , AttachCurrentThread , & mut env_ptr , & mut args as * mut sys :: JavaVMAttachArgs as * mut core :: ffi :: c_void) ; jni_error_code_to_result (res) ? ; ATTACHED_THREADS . fetch_add (1 , Ordering :: SeqCst) ; debug ! ("Attached thread {} ({:?}). {} threads attached" , thread . name () . unwrap_or_default () , thread . id () , ATTACHED_THREADS . load (Ordering :: SeqCst)) ; Ok (env_ptr as * mut sys :: JNIEnv) }
};
}
