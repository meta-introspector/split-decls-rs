// Generated macro for impl_403 (impl)
macro_rules! Depcrate_objects_jiteratorimpl_403 {
() => {
// Module: crate::objects::jiterator
// Provides: {"impl_403"}
// Dependencies: {}
impl JIteratorAPI { fn get < 'any_local > (env : & Env < '_ > , loader_context : & LoaderContext < 'any_local , '_ > ,) -> Result < & 'static Self > { static JITERATOR_API : OnceCell < JIteratorAPI > = OnceCell :: new () ; JITERATOR_API . get_or_try_init (| | { env . with_local_frame (DEFAULT_LOCAL_FRAME_CAPACITY , | env | { let class = loader_context . load_class_for_type :: < JIterator > (true , env) ? ; let class = env . new_global_ref (& class) . unwrap () ; let has_next_method = env . get_method_id (& class , c"hasNext" , c"()Z") ? ; let next_method = env . get_method_id (& class , c"next" , c"()Ljava/lang/Object;") ? ; let remove_method = env . get_method_id (& class , c"remove" , c"()V") ? ; Ok (Self { class , has_next_method , next_method , remove_method , }) }) }) } }
};
}
