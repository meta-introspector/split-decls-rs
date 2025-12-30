// Generated macro for impl_479 (impl)
macro_rules! Depcrate_objects_jthreadimpl_479 {
() => {
// Module: crate::objects::jthread
// Provides: {"impl_479"}
// Dependencies: {}
unsafe impl Reference for JThread < '_ > { type Kind < 'env > = JThread < 'env > ; type GlobalKind = JThread < 'static > ; fn as_raw (& self) -> jobject { self . 0 . as_raw () } fn class_name () -> Cow < 'static , JNIStr > { Cow :: Borrowed (JNIStr :: from_cstr (c"java.lang.Thread")) } fn lookup_class < 'caller > (env : & Env < '_ > , _loader_context : LoaderContext ,) -> crate :: errors :: Result < impl Deref < Target = Global < JClass < 'static > > > + 'caller > { let api = JThreadAPI :: get (env) ? ; Ok (& api . class) } unsafe fn kind_from_raw < 'env > (local_ref : jobject) -> Self :: Kind < 'env > { JThread (JObject :: kind_from_raw (local_ref)) } unsafe fn global_kind_from_raw (global_ref : jobject) -> Self :: GlobalKind { JThread (JObject :: global_kind_from_raw (global_ref)) } }
};
}
