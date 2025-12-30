// Generated macro for impl_292 (impl)
macro_rules! Depcrate_objects_jthrowableimpl_292 {
() => {
// Module: crate::objects::jthrowable
// Provides: {"impl_292"}
// Dependencies: {}
unsafe impl Reference for JThrowable < '_ > { type Kind < 'env > = JThrowable < 'env > ; type GlobalKind = JThrowable < 'static > ; fn as_raw (& self) -> jobject { self . 0 . as_raw () } fn class_name () -> Cow < 'static , JNIStr > { Cow :: Borrowed (JNIStr :: from_cstr (c"java.lang.Throwable")) } fn lookup_class < 'caller > (env : & Env < '_ > , loader_context : LoaderContext ,) -> crate :: errors :: Result < impl Deref < Target = Global < JClass < 'static > > > + 'caller > { let api = JThrowableAPI :: get (env , & loader_context) ? ; Ok (& api . class) } unsafe fn kind_from_raw < 'env > (local_ref : jobject) -> Self :: Kind < 'env > { JThrowable (JObject :: kind_from_raw (local_ref)) } unsafe fn global_kind_from_raw (global_ref : jobject) -> Self :: GlobalKind { JThrowable (JObject :: global_kind_from_raw (global_ref)) } }
};
}
