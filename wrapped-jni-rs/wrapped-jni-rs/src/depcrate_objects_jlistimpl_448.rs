// Generated macro for impl_448 (impl)
macro_rules! Depcrate_objects_jlistimpl_448 {
() => {
// Module: crate::objects::jlist
// Provides: {"impl_448"}
// Dependencies: {}
unsafe impl Reference for JList < '_ > { type Kind < 'env > = JList < 'env > ; type GlobalKind = JList < 'static > ; fn as_raw (& self) -> jobject { self . 0 . as_raw () } fn class_name () -> Cow < 'static , JNIStr > { Cow :: Borrowed (JNIStr :: from_cstr (c"java.util.List")) } fn lookup_class < 'caller > (env : & Env < '_ > , loader_context : LoaderContext ,) -> crate :: errors :: Result < impl Deref < Target = Global < JClass < 'static > > > + 'caller > { let api = JListAPI :: get (env , & loader_context) ? ; Ok (& api . class) } unsafe fn kind_from_raw < 'env > (local_ref : jobject) -> Self :: Kind < 'env > { JList (JObject :: kind_from_raw (local_ref)) } unsafe fn global_kind_from_raw (global_ref : jobject) -> Self :: GlobalKind { JList (JObject :: global_kind_from_raw (global_ref)) } }
};
}
