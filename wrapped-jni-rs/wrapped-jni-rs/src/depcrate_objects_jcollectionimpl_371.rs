// Generated macro for impl_371 (impl)
macro_rules! Depcrate_objects_jcollectionimpl_371 {
() => {
// Module: crate::objects::jcollection
// Provides: {"impl_371"}
// Dependencies: {}
impl JCollectionAPI { fn get < 'any_local > (env : & Env < '_ > , loader_context : & LoaderContext < 'any_local , '_ > ,) -> Result < & 'static Self > { static JCOLLECTION_API : OnceCell < JCollectionAPI > = OnceCell :: new () ; JCOLLECTION_API . get_or_try_init (| | { env . with_local_frame (DEFAULT_LOCAL_FRAME_CAPACITY , | env | { let class = loader_context . load_class_for_type :: < JCollection > (true , env) ? ; let class = env . new_global_ref (& class) . unwrap () ; let add_method = env . get_method_id (& class , c"add" , c"(Ljava/lang/Object;)Z") ? ; let remove_method = env . get_method_id (& class , c"remove" , c"(Ljava/lang/Object;)Z") ? ; let clear_method = env . get_method_id (& class , c"clear" , c"()V") ? ; let contains_method = env . get_method_id (& class , c"contains" , c"(Ljava/lang/Object;)Z") ? ; let size_method = env . get_method_id (& class , c"size" , c"()I") ? ; let is_empty_method = env . get_method_id (& class , c"isEmpty" , c"()Z") ? ; let iterator_method = env . get_method_id (& class , c"iterator" , c"()Ljava/util/Iterator;") ? ; Ok (Self { class , add_method , remove_method , clear_method , contains_method , size_method , is_empty_method , iterator_method , }) }) }) } }
};
}
