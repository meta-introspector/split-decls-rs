// Generated macro for impl_388 (impl)
macro_rules! Depcrate_objects_jsetimpl_388 {
() => {
// Module: crate::objects::jset
// Provides: {"impl_388"}
// Dependencies: {}
impl JSetAPI { fn get < 'any_local > (env : & Env < '_ > , loader_context : & LoaderContext < 'any_local , '_ > ,) -> Result < & 'static Self > { static JSET_API : OnceCell < JSetAPI > = OnceCell :: new () ; JSET_API . get_or_try_init (| | { env . with_local_frame (DEFAULT_LOCAL_FRAME_CAPACITY , | env | { let class = loader_context . load_class_for_type :: < JSet > (true , env) ? ; let class = env . new_global_ref (& class) . unwrap () ; Ok (Self { class }) }) }) } }
};
}
