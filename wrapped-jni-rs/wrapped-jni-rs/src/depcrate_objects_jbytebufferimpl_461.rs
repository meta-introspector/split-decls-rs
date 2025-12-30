// Generated macro for impl_461 (impl)
macro_rules! Depcrate_objects_jbytebufferimpl_461 {
() => {
// Module: crate::objects::jbytebuffer
// Provides: {"impl_461"}
// Dependencies: {}
impl JByteBufferAPI { fn get < 'any_local > (env : & Env < '_ > , loader_context : & LoaderContext < 'any_local , '_ > ,) -> Result < & 'static Self > { static JBYTEBUFFER_API : OnceCell < JByteBufferAPI > = OnceCell :: new () ; JBYTEBUFFER_API . get_or_try_init (| | { env . with_local_frame (DEFAULT_LOCAL_FRAME_CAPACITY , | env | { let class = loader_context . load_class_for_type :: < JByteBuffer > (false , env) ? ; let class = env . new_global_ref (& class) . unwrap () ; Ok (Self { class }) }) }) } }
};
}
