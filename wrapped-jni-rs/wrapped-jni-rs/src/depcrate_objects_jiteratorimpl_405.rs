// Generated macro for impl_405 (impl)
macro_rules! Depcrate_objects_jiteratorimpl_405 {
() => {
// Module: crate::objects::jiterator
// Provides: {"impl_405"}
// Dependencies: {}
unsafe impl Reference for JIterator < '_ > { type Kind < 'env > = JIterator < 'env > ; type GlobalKind = JIterator < 'static > ; fn as_raw (& self) -> jobject { self . 0 . as_raw () } fn class_name () -> Cow < 'static , JNIStr > { Cow :: Borrowed (JNIStr :: from_cstr (c"java.util.Iterator")) } fn lookup_class < 'caller > (env : & Env < '_ > , loader_context : LoaderContext ,) -> crate :: errors :: Result < impl Deref < Target = Global < JClass < 'static > > > + 'caller > { let api = JIteratorAPI :: get (env , & loader_context) ? ; Ok (& api . class) } unsafe fn kind_from_raw < 'env > (local_ref : jobject) -> Self :: Kind < 'env > { JIterator (JObject :: kind_from_raw (local_ref)) } unsafe fn global_kind_from_raw (global_ref : jobject) -> Self :: GlobalKind { JIterator (JObject :: global_kind_from_raw (global_ref)) } }
};
}
