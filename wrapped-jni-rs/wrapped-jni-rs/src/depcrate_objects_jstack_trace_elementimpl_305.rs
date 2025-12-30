// Generated macro for impl_305 (impl)
macro_rules! Depcrate_objects_jstack_trace_elementimpl_305 {
() => {
// Module: crate::objects::jstack_trace_element
// Provides: {"impl_305"}
// Dependencies: {}
impl JStackTraceElementAPI { fn get (env : & Env < '_ > , loader_context : & LoaderContext < '_ , '_ >) -> Result < & 'static Self > { static JSTACK_TRACE_ELEMENT_API : OnceCell < JStackTraceElementAPI > = OnceCell :: new () ; JSTACK_TRACE_ELEMENT_API . get_or_try_init (| | { env . with_local_frame (DEFAULT_LOCAL_FRAME_CAPACITY , | env | { let class = loader_context . load_class_for_type :: < JStackTraceElement > (false , env) ? ; let class = env . new_global_ref (& class) . unwrap () ; let get_class_name_method = env . get_method_id (& class , c"getClassName" , c"()Ljava/lang/String;") . expect ("StackTraceElement.getClassName method not found") ; let get_file_name_method = env . get_method_id (& class , c"getFileName" , c"()Ljava/lang/String;") . expect ("StackTraceElement.getFileName method not found") ; let get_line_number_method = env . get_method_id (& class , c"getLineNumber" , c"()I") . expect ("StackTraceElement.getLineNumber method not found") ; let get_method_name_method = env . get_method_id (& class , c"getMethodName" , c"()Ljava/lang/String;") . expect ("StackTraceElement.getMethodName method not found") ; let is_native_method = env . get_method_id (& class , c"isNative" , c"()Z") . expect ("StackTraceElement.isNative method not found") ; let to_string_method = env . get_method_id (& class , c"toString" , c"()Ljava/lang/String;") . expect ("StackTraceElement.toString method not found") ; Ok (Self { class , get_class_name_method , get_file_name_method , get_line_number_method , get_method_name_method , is_native_method , to_string_method , }) }) }) } }
};
}
