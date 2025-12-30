// Generated macro for impl_420 (impl)
macro_rules! Depcrate_objects_jmapimpl_420 {
() => {
// Module: crate::objects::jmap
// Provides: {"impl_420"}
// Dependencies: {}
unsafe impl Reference for JMap < '_ > { type Kind < 'env > = JMap < 'env > ; type GlobalKind = JMap < 'static > ; fn as_raw (& self) -> jobject { self . 0 . as_raw () } fn class_name () -> Cow < 'static , JNIStr > { Cow :: Borrowed (JNIStr :: from_cstr (c"java.util.Map")) } fn lookup_class < 'caller > (env : & Env < '_ > , loader_context : LoaderContext ,) -> crate :: errors :: Result < impl Deref < Target = Global < JClass < 'static > > > + 'caller > { let api = JMapAPI :: get (env , & loader_context) ? ; Ok (& api . class) } unsafe fn kind_from_raw < 'env > (local_ref : jobject) -> Self :: Kind < 'env > { JMap (JObject :: kind_from_raw (local_ref)) } unsafe fn global_kind_from_raw (global_ref : jobject) -> Self :: GlobalKind { JMap (JObject :: global_kind_from_raw (global_ref)) } }
};
}
