// Generated macro for impl_446 (impl)
macro_rules! Depcrate_objects_jlistimpl_446 {
() => {
// Module: crate::objects::jlist
// Provides: {"impl_446"}
// Dependencies: {}
impl JListAPI { fn get < 'any_local > (env : & Env < '_ > , loader_context : & LoaderContext < 'any_local , '_ > ,) -> Result < & 'static Self > { static JLIST_API : OnceCell < JListAPI > = OnceCell :: new () ; JLIST_API . get_or_try_init (| | { env . with_local_frame (DEFAULT_LOCAL_FRAME_CAPACITY , | env | { let class = loader_context . load_class_for_type :: < JList > (true , env) ? ; let class = env . new_global_ref (& class) . unwrap () ; let get_method = env . get_method_id (& class , c"get" , c"(I)Ljava/lang/Object;") ? ; let add_idx_method = env . get_method_id (& class , c"add" , c"(ILjava/lang/Object;)V") ? ; let remove_method = env . get_method_id (& class , c"remove" , c"(I)Ljava/lang/Object;") ? ; Ok (Self { class , get_method , add_idx_method , remove_method , }) }) }) } }
};
}
