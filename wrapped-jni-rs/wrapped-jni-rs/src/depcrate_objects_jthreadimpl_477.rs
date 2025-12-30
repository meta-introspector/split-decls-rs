// Generated macro for impl_477 (impl)
macro_rules! Depcrate_objects_jthreadimpl_477 {
() => {
// Module: crate::objects::jthread
// Provides: {"impl_477"}
// Dependencies: {}
impl JThreadAPI { fn get (env : & Env < '_ >) -> Result < & 'static Self > { static JTHREAD_API : OnceCell < JThreadAPI > = OnceCell :: new () ; JTHREAD_API . get_or_try_init (| | { env . with_local_frame (DEFAULT_LOCAL_FRAME_CAPACITY , | env | { let class = env . find_class (JNIStr :: from_cstr (c"java/lang/Thread")) ? ; let class = env . new_global_ref (& class) . unwrap () ; let current_thread_method = env . get_static_method_id (& class , c"currentThread" , c"()Ljava/lang/Thread;") . expect ("Thread.currentThread method not found") ; let get_name_method = env . get_method_id (& class , c"getName" , c"()Ljava/lang/String;") . expect ("Thread.getName method not found") ; let set_name_method = env . get_method_id (& class , c"setName" , c"(Ljava/lang/String;)V") . expect ("Thread.setName method not found") ; let get_id_method = env . get_method_id (& class , c"getId" , c"()J") . expect ("Thread.getId method not found") ; let get_context_class_loader_method = env . get_method_id (& class , c"getContextClassLoader" , c"()Ljava/lang/ClassLoader;" ,) . expect ("Thread.getContextClassLoader method not found") ; let set_context_class_loader_method = env . get_method_id (& class , c"setContextClassLoader" , c"(Ljava/lang/ClassLoader;)V" ,) . expect ("Thread.setContextClassLoader method not found") ; Ok (Self { class , current_thread_method , get_name_method , set_name_method , get_id_method , get_context_class_loader_method , set_context_class_loader_method , }) }) }) } }
};
}
