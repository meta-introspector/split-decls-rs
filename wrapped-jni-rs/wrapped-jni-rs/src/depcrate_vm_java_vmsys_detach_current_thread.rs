// Generated macro for sys_detach_current_thread (function)
macro_rules! Depcrate_vm_java_vmsys_detach_current_thread {
() => {
// Module: crate::vm::java_vm
// Provides: {"sys_detach_current_thread"}
// Dependencies: {}
# [doc = " Detach a thread, asserting that we own the current attachment and have a valid `Env` pointer"] # [doc = ""] # [doc = " Although `DetachCurrentThread` is part of the `JavaVM` \"invocation\" API and doesn't require a"] # [doc = " `Env` pointer, we want to constrain this code to only ever detach threads if we own the"] # [doc = " current attachment."] unsafe fn sys_detach_current_thread (env_ptr : * mut jni_sys :: JNIEnv , thread : & Thread) -> Result < () > { assert_eq ! (JavaVM :: thread_attach_guard_level () , 0) ; unsafe { let mut guard = AttachGuard :: from_unowned (env_ptr) ; let env = guard . borrow_env_mut () ; let vm = env . get_java_vm () ; let vm_get_env_guard = vm . sys_get_env_attachment () ? ; if vm_get_env_guard . env . raw != env_ptr { return Err (Error :: JniCall (JniError :: InvalidArguments)) ; } java_vm_call_unchecked ! (vm , v1_1 , DetachCurrentThread) ; } ATTACHED_THREADS . fetch_sub (1 , Ordering :: SeqCst) ; debug ! ("Detached thread {} ({:?}). {} threads remain attached" , thread . name () . unwrap_or_default () , thread . id () , ATTACHED_THREADS . load (Ordering :: SeqCst)) ; Ok (()) }
};
}
