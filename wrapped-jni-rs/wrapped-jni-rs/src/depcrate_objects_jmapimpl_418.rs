// Generated macro for impl_418 (impl)
macro_rules! Depcrate_objects_jmapimpl_418 {
() => {
// Module: crate::objects::jmap
// Provides: {"impl_418"}
// Dependencies: {}
impl JMapAPI { fn get < 'any_local > (env : & Env < '_ > , loader_context : & LoaderContext < 'any_local , '_ > ,) -> Result < & 'static Self > { static JMAP_API : OnceCell < JMapAPI > = OnceCell :: new () ; JMAP_API . get_or_try_init (| | { env . with_local_frame (DEFAULT_LOCAL_FRAME_CAPACITY , | env | { let class = loader_context . load_class_for_type :: < JMap > (true , env) ? ; let class = env . new_global_ref (& class) . unwrap () ; let get_method = env . get_method_id (& class , c"get" , c"(Ljava/lang/Object;)Ljava/lang/Object;") ? ; let put_method = env . get_method_id (& class , c"put" , c"(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;" ,) ? ; let remove_method = env . get_method_id (& class , c"remove" , c"(Ljava/lang/Object;)Ljava/lang/Object;" ,) ? ; let entry_set_method = env . get_method_id (& class , c"entrySet" , c"()Ljava/util/Set;") ? ; Ok (Self { class , get_method , put_method , remove_method , entry_set_method , }) }) }) } }
};
}
