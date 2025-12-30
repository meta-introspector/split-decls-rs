// Generated macro for impl_337 (impl)
macro_rules! Depcrate_objects_jclass_loaderimpl_337 {
() => {
// Module: crate::objects::jclass_loader
// Provides: {"impl_337"}
// Dependencies: {}
impl JClassLoaderAPI { fn get (env : & Env < '_ >) -> Result < & 'static Self > { static JCLASS_LOADER_API : OnceCell < JClassLoaderAPI > = OnceCell :: new () ; JCLASS_LOADER_API . get_or_try_init (| | { env . with_local_frame (DEFAULT_LOCAL_FRAME_CAPACITY , | env | { let class = env . find_class (c"java/lang/ClassLoader") ? ; let class = env . new_global_ref (& class) . unwrap () ; let load_class_method = env . get_method_id (& class , c"loadClass" , c"(Ljava/lang/String;)Ljava/lang/Class;" ,) ? ; Ok (Self { class , load_class_method , }) }) }) } }
};
}
