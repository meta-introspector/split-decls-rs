// Generated macro for impl_321 (impl)
macro_rules! Depcrate_objects_jclassimpl_321 {
() => {
// Module: crate::objects::jclass
// Provides: {"impl_321"}
// Dependencies: {}
impl JClassAPI { pub fn get (env : & Env < '_ >) -> Result < & 'static Self > { static JCLASS_API : OnceCell < JClassAPI > = OnceCell :: new () ; JCLASS_API . get_or_try_init (| | { env . with_local_frame (DEFAULT_LOCAL_FRAME_CAPACITY , | env | { let class = env . find_class (JNIStr :: from_cstr (c"java/lang/Class")) ? ; let class = env . new_global_ref (class) ? ; let get_class_loader_method = env . get_method_id (& class , c"getClassLoader" , c"()Ljava/lang/ClassLoader;") ? ; let for_name_method = env . get_static_method_id (& class , c"forName" , c"(Ljava/lang/String;)Ljava/lang/Class;" ,) ? ; let for_name_with_loader_method = env . get_static_method_id (& class , c"forName" , c"(Ljava/lang/String;ZLjava/lang/ClassLoader;)Ljava/lang/Class;" ,) ? ; Ok (Self { class , get_class_loader_method , for_name_method , for_name_with_loader_method , }) }) }) } }
};
}
