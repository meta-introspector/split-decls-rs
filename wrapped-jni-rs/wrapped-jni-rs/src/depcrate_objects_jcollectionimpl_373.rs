// Generated macro for impl_373 (impl)
macro_rules! Depcrate_objects_jcollectionimpl_373 {
() => {
// Module: crate::objects::jcollection
// Provides: {"impl_373"}
// Dependencies: {}
unsafe impl Reference for JCollection < '_ > { type Kind < 'env > = JCollection < 'env > ; type GlobalKind = JCollection < 'static > ; fn as_raw (& self) -> jobject { self . 0 . as_raw () } fn class_name () -> Cow < 'static , JNIStr > { Cow :: Borrowed (JNIStr :: from_cstr (c"java.util.Collection")) } fn lookup_class < 'caller > (env : & Env < '_ > , loader_context : LoaderContext ,) -> crate :: errors :: Result < impl Deref < Target = Global < JClass < 'static > > > + 'caller > { let api = JCollectionAPI :: get (env , & loader_context) ? ; Ok (& api . class) } unsafe fn kind_from_raw < 'env > (local_ref : jobject) -> Self :: Kind < 'env > { JCollection (JObject :: kind_from_raw (local_ref)) } unsafe fn global_kind_from_raw (global_ref : jobject) -> Self :: GlobalKind { JCollection (JObject :: global_kind_from_raw (global_ref)) } }
};
}
