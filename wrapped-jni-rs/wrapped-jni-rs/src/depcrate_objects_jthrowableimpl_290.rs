// Generated macro for impl_290 (impl)
macro_rules! Depcrate_objects_jthrowableimpl_290 {
() => {
// Module: crate::objects::jthrowable
// Provides: {"impl_290"}
// Dependencies: {}
impl JThrowableAPI { fn get < 'any_local > (env : & Env < 'any_local > , loader_context : & LoaderContext < 'any_local , '_ > ,) -> Result < & 'static Self > { static JTHROWABLE_API : OnceCell < JThrowableAPI > = OnceCell :: new () ; JTHROWABLE_API . get_or_try_init (| | { env . with_local_frame (DEFAULT_LOCAL_FRAME_CAPACITY , | env | { let class = loader_context . load_class_for_type :: < JThrowable > (true , env) ? ; let class = env . new_global_ref (& class) . unwrap () ; let get_message_method = env . get_method_id (& class , c"getMessage" , c"()Ljava/lang/String;") . expect ("JThrowable.getMessage method not found") ; let get_cause_method = env . get_method_id (& class , c"getCause" , c"()Ljava/lang/Throwable;") . expect ("JThrowable.getCause method not found") ; let get_stack_trace_method = env . get_method_id (& class , c"getStackTrace" , c"()[Ljava/lang/StackTraceElement;" ,) . expect ("JThrowable.getStackTrace method not found") ; Ok (Self { class , get_message_method , get_cause_method , get_stack_trace_method , }) }) }) } }
};
}
