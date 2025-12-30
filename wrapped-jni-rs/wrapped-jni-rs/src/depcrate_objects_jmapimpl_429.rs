// Generated macro for impl_429 (impl)
macro_rules! Depcrate_objects_jmapimpl_429 {
() => {
// Module: crate::objects::jmap
// Provides: {"impl_429"}
// Dependencies: {}
unsafe impl Reference for JMapEntry < '_ > { type Kind < 'env > = JMapEntry < 'env > ; type GlobalKind = JMapEntry < 'static > ; fn as_raw (& self) -> jobject { self . 0 . as_raw () } fn class_name () -> Cow < 'static , JNIStr > { Cow :: Borrowed (JNIStr :: from_cstr (c"java.util.Map$Entry")) } fn lookup_class < 'caller > (env : & Env < '_ > , loader_context : LoaderContext ,) -> crate :: errors :: Result < impl Deref < Target = Global < JClass < 'static > > > + 'caller > { let api = JMapEntryAPI :: get (env , & loader_context) ? ; Ok (& api . class) } unsafe fn kind_from_raw < 'env > (local_ref : jobject) -> Self :: Kind < 'env > { JMapEntry (JObject :: kind_from_raw (local_ref)) } unsafe fn global_kind_from_raw (global_ref : jobject) -> Self :: GlobalKind { JMapEntry (JObject :: global_kind_from_raw (global_ref)) } }
};
}
