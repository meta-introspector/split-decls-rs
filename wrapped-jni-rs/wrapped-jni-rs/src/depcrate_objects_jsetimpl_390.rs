// Generated macro for impl_390 (impl)
macro_rules! Depcrate_objects_jsetimpl_390 {
() => {
// Module: crate::objects::jset
// Provides: {"impl_390"}
// Dependencies: {}
unsafe impl Reference for JSet < '_ > { type Kind < 'env > = JSet < 'env > ; type GlobalKind = JSet < 'static > ; fn as_raw (& self) -> jobject { self . 0 . as_raw () } fn class_name () -> Cow < 'static , JNIStr > { Cow :: Borrowed (JNIStr :: from_cstr (c"java.util.Set")) } fn lookup_class < 'caller > (env : & Env < '_ > , loader_context : LoaderContext ,) -> crate :: errors :: Result < impl Deref < Target = Global < JClass < 'static > > > + 'caller > { let api = JSetAPI :: get (env , & loader_context) ? ; Ok (& api . class) } unsafe fn kind_from_raw < 'env > (local_ref : jobject) -> Self :: Kind < 'env > { JSet (JObject :: kind_from_raw (local_ref)) } unsafe fn global_kind_from_raw (global_ref : jobject) -> Self :: GlobalKind { JSet (JObject :: global_kind_from_raw (global_ref)) } }
};
}
