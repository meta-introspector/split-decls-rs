// Generated macro for impl_276 (impl)
macro_rules! Depcrate_objects_jobjectimpl_276 {
() => {
// Module: crate::objects::jobject
// Provides: {"impl_276"}
// Dependencies: {}
unsafe impl Reference for JObject < '_ > { type Kind < 'env > = JObject < 'env > ; type GlobalKind = JObject < 'static > ; fn as_raw (& self) -> jobject { self . as_raw () } fn class_name () -> Cow < 'static , JNIStr > { Cow :: Borrowed (JNIStr :: from_cstr (c"java.lang.Object")) } fn lookup_class < 'caller > (env : & Env < '_ > , _loader_context : LoaderContext ,) -> crate :: errors :: Result < impl Deref < Target = Global < JClass < 'static > > > + 'caller > { let api = JObjectAPI :: get (env) ? ; Ok (& api . class) } unsafe fn kind_from_raw < 'env > (local_ref : jobject) -> Self :: Kind < 'env > { JObject { internal : local_ref , lifetime : PhantomData , } } unsafe fn global_kind_from_raw (global_ref : jobject) -> Self :: GlobalKind { JObject { internal : global_ref , lifetime : PhantomData , } } }
};
}
