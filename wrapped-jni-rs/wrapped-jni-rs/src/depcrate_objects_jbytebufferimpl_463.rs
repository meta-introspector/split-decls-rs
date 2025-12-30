// Generated macro for impl_463 (impl)
macro_rules! Depcrate_objects_jbytebufferimpl_463 {
() => {
// Module: crate::objects::jbytebuffer
// Provides: {"impl_463"}
// Dependencies: {}
unsafe impl Reference for JByteBuffer < '_ > { type Kind < 'env > = JByteBuffer < 'env > ; type GlobalKind = JByteBuffer < 'static > ; fn as_raw (& self) -> jobject { self . 0 . as_raw () } fn class_name () -> Cow < 'static , JNIStr > { Cow :: Borrowed (JNIStr :: from_cstr (c"[Ljava.nio.ByteBuffer;")) } fn lookup_class < 'caller > (env : & Env < '_ > , loader_context : LoaderContext ,) -> crate :: errors :: Result < impl Deref < Target = Global < JClass < 'static > > > + 'caller > { let api = JByteBufferAPI :: get (env , & loader_context) ? ; Ok (& api . class) } unsafe fn kind_from_raw < 'env > (local_ref : jobject) -> Self :: Kind < 'env > { JByteBuffer (JObject :: kind_from_raw (local_ref)) } unsafe fn global_kind_from_raw (global_ref : jobject) -> Self :: GlobalKind { JByteBuffer (JObject :: global_kind_from_raw (global_ref)) } }
};
}
