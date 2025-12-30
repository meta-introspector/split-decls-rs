// Generated macro for impl_273 (impl)
macro_rules! Depcrate_objects_jobjectimpl_273 {
() => {
// Module: crate::objects::jobject
// Provides: {"impl_273"}
// Dependencies: {}
impl JObjectAPI { fn get (env : & Env < '_ >) -> Result < & 'static Self > { static JOBJECT_API : OnceCell < JObjectAPI > = OnceCell :: new () ; JOBJECT_API . get_or_try_init (| | { env . with_local_frame (DEFAULT_LOCAL_FRAME_CAPACITY , | env | { let class = env . find_class (JNIStr :: from_cstr (c"java/lang/Object")) ? ; let class = env . new_global_ref (class) ? ; Ok (JObjectAPI { class }) }) }) } }
};
}
