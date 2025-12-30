// Generated macro for impl_357 (impl)
macro_rules! Depcrate_objects_jstringimpl_357 {
() => {
// Module: crate::objects::jstring
// Provides: {"impl_357"}
// Dependencies: {}
unsafe impl Reference for JString < '_ > { type Kind < 'env > = JString < 'env > ; type GlobalKind = JString < 'static > ; fn as_raw (& self) -> jobject { self . 0 . as_raw () } fn class_name () -> Cow < 'static , JNIStr > { Cow :: Borrowed (JNIStr :: from_cstr (c"java.lang.String")) } fn lookup_class < 'caller > (env : & Env < '_ > , loader_context : LoaderContext ,) -> crate :: errors :: Result < impl Deref < Target = Global < JClass < 'static > > > + 'caller > { let api = JStringAPI :: get (env , & loader_context) ? ; Ok (& api . class) } unsafe fn kind_from_raw < 'env > (local_ref : jobject) -> Self :: Kind < 'env > { JString (JObject :: kind_from_raw (local_ref)) } unsafe fn global_kind_from_raw (global_ref : jobject) -> Self :: GlobalKind { JString (JObject :: global_kind_from_raw (global_ref)) } }
};
}
