// Generated macro for impl_307 (impl)
macro_rules! Depcrate_objects_jstack_trace_elementimpl_307 {
() => {
// Module: crate::objects::jstack_trace_element
// Provides: {"impl_307"}
// Dependencies: {}
unsafe impl Reference for JStackTraceElement < '_ > { type Kind < 'env > = JStackTraceElement < 'env > ; type GlobalKind = JStackTraceElement < 'static > ; fn as_raw (& self) -> jobject { self . 0 . as_raw () } fn class_name () -> Cow < 'static , JNIStr > { Cow :: Borrowed (JNIStr :: from_cstr (c"java.lang.StackTraceElement")) } fn lookup_class < 'caller > (env : & Env < '_ > , loader_context : LoaderContext ,) -> crate :: errors :: Result < impl Deref < Target = Global < JClass < 'static > > > + 'caller > { let api = JStackTraceElementAPI :: get (env , & loader_context) ? ; Ok (& api . class) } unsafe fn kind_from_raw < 'env > (local_ref : jobject) -> Self :: Kind < 'env > { JStackTraceElement (JObject :: kind_from_raw (local_ref)) } unsafe fn global_kind_from_raw (global_ref : jobject) -> Self :: GlobalKind { JStackTraceElement (JObject :: global_kind_from_raw (global_ref)) } }
};
}
