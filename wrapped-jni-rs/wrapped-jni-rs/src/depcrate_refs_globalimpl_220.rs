// Generated macro for impl_220 (impl)
macro_rules! Depcrate_refs_globalimpl_220 {
() => {
// Module: crate::refs::global
// Provides: {"impl_220"}
// Dependencies: {}
unsafe impl < T > Reference for Global < T > where T : Into < JObject < 'static > > + AsRef < JObject < 'static > > + Default + Reference + Send + Sync , { type Kind < 'env > = T :: Kind < 'env > ; type GlobalKind = T :: GlobalKind ; fn as_raw (& self) -> jobject { self . obj . as_raw () } fn class_name () -> Cow < 'static , JNIStr > { T :: class_name () } fn lookup_class < 'caller > (env : & Env < '_ > , loader_context : LoaderContext ,) -> crate :: errors :: Result < impl Deref < Target = Global < JClass < 'static > > > + 'caller > { T :: lookup_class (env , loader_context) } unsafe fn kind_from_raw < 'env > (local_ref : jobject) -> Self :: Kind < 'env > { T :: kind_from_raw :: < 'env > (local_ref) } unsafe fn global_kind_from_raw (global_ref : jobject) -> Self :: GlobalKind { T :: global_kind_from_raw (global_ref) } }
};
}
