// Generated macro for impl_427 (impl)
macro_rules! Depcrate_objects_jmapimpl_427 {
() => {
// Module: crate::objects::jmap
// Provides: {"impl_427"}
// Dependencies: {}
impl JMapEntryAPI { fn get < 'any_local > (env : & Env < '_ > , loader_context : & LoaderContext < 'any_local , '_ > ,) -> Result < & 'static Self > { static JMAPENTRY_API : OnceCell < JMapEntryAPI > = OnceCell :: new () ; JMAPENTRY_API . get_or_try_init (| | { env . with_local_frame (DEFAULT_LOCAL_FRAME_CAPACITY , | env | { let class = loader_context . load_class_for_type :: < JMapEntry > (true , env) ? ; let class = env . new_global_ref (& class) . unwrap () ; let get_key_method = env . get_method_id (& class , c"getKey" , c"()Ljava/lang/Object;") ? ; let get_value_method = env . get_method_id (& class , c"getValue" , c"()Ljava/lang/Object;") ? ; let set_value_method = env . get_method_id (& class , c"setValue" , c"(Ljava/lang/Object;)Ljava/lang/Object;" ,) ? ; Ok (Self { class , get_key_method , get_value_method , set_value_method , }) }) }) } }
};
}
