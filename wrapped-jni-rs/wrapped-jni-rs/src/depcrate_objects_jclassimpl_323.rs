// Generated macro for impl_323 (impl)
macro_rules! Depcrate_objects_jclassimpl_323 {
() => {
// Module: crate::objects::jclass
// Provides: {"impl_323"}
// Dependencies: {}
unsafe impl Reference for JClass < '_ > { type Kind < 'env > = JClass < 'env > ; type GlobalKind = JClass < 'static > ; fn as_raw (& self) -> jobject { self . 0 . as_raw () } fn class_name () -> Cow < 'static , JNIStr > { Cow :: Borrowed (JNIStr :: from_cstr (c"java.lang.Class")) } fn lookup_class < 'caller > (env : & Env < '_ > , _loader_context : LoaderContext ,) -> crate :: errors :: Result < impl Deref < Target = Global < JClass < 'static > > > + 'caller > { let api = JClassAPI :: get (env) ? ; Ok (& api . class) } unsafe fn kind_from_raw < 'env > (local_ref : jobject) -> Self :: Kind < 'env > { JClass (JObject :: kind_from_raw (local_ref)) } unsafe fn global_kind_from_raw (global_ref : jobject) -> Self :: GlobalKind { JClass (JObject :: global_kind_from_raw (global_ref)) } }
};
}
