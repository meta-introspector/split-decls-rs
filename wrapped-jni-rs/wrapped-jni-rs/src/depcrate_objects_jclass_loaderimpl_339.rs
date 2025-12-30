// Generated macro for impl_339 (impl)
macro_rules! Depcrate_objects_jclass_loaderimpl_339 {
() => {
// Module: crate::objects::jclass_loader
// Provides: {"impl_339"}
// Dependencies: {}
unsafe impl Reference for JClassLoader < '_ > { type Kind < 'env > = JClassLoader < 'env > ; type GlobalKind = JClassLoader < 'static > ; fn as_raw (& self) -> jobject { self . 0 . as_raw () } fn class_name () -> Cow < 'static , JNIStr > { Cow :: Borrowed (JNIStr :: from_cstr (c"java.lang.ClassLoader")) } fn lookup_class < 'caller > (env : & Env < '_ > , _loader_context : LoaderContext ,) -> crate :: errors :: Result < impl Deref < Target = Global < JClass < 'static > > > + 'caller > { let api = JClassLoaderAPI :: get (env) ? ; Ok (& api . class) } unsafe fn kind_from_raw < 'env > (local_ref : jobject) -> Self :: Kind < 'env > { JClassLoader (JObject :: kind_from_raw (local_ref)) } unsafe fn global_kind_from_raw (global_ref : jobject) -> Self :: GlobalKind { JClassLoader (JObject :: global_kind_from_raw (global_ref)) } }
};
}
