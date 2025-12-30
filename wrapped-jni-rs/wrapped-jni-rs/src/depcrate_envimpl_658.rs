// Generated macro for impl_658 (impl)
macro_rules! Depcrate_envimpl_658 {
() => {
// Module: crate::env
// Provides: {"impl_658"}
// Dependencies: {}
impl Drop for MonitorGuard < '_ > { fn drop (& mut self) { JavaVM :: singleton () . expect ("JavaVM singleton must be initialized") . with_top_local_frame (| env | -> crate :: errors :: Result < () > { let res = unsafe { jni_call_unchecked ! (env , v1_1 , MonitorExit , self . obj) } ; if let Err (err) = jni_error_code_to_result (res) { log :: error ! ("error releasing java monitor: {err}") ; } Ok (()) }) . expect ("MonitorGuard dropped on detached thread") ; } }
};
}
