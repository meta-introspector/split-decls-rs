// Generated macro for impl_355 (impl)
macro_rules! Depcrate_objects_jstringimpl_355 {
() => {
// Module: crate::objects::jstring
// Provides: {"impl_355"}
// Dependencies: {}
impl JStringAPI { fn get < 'any_local > (env : & Env < '_ > , loader_context : & LoaderContext < 'any_local , '_ > ,) -> Result < & 'static Self > { static JSTRING_API : OnceCell < JStringAPI > = OnceCell :: new () ; JSTRING_API . get_or_try_init (| | { env . with_local_frame (DEFAULT_LOCAL_FRAME_CAPACITY , | env | { let class = loader_context . load_class_for_type :: < JString > (true , env) ? ; let class = env . new_global_ref (& class) . unwrap () ; let intern_method = env . get_method_id (& class , c"intern" , c"()Ljava/lang/String;") . expect ("JString.intern method not found") ; Ok (Self { class , intern_method , }) }) }) } }
};
}
