// Generated macro for impl_257 (impl)
macro_rules! Depcrate_refs_autoimpl_257 {
() => {
// Module: crate::refs::auto
// Provides: {"impl_257"}
// Dependencies: {}
unsafe impl < 'local , T > Reference for Auto < 'local , T > where T : Reference + Into < JObject < 'local > > , { type Kind < 'env > = T :: Kind < 'env > ; type GlobalKind = T :: GlobalKind ; fn as_raw (& self) -> jobject { self . obj . as_raw () } fn class_name () -> Cow < 'static , JNIStr > { T :: class_name () } fn lookup_class < 'caller > (env : & Env < '_ > , loader_context : LoaderContext ,) -> crate :: errors :: Result < impl Deref < Target = Global < JClass < 'static > > > + 'caller > { T :: lookup_class (env , loader_context) } unsafe fn kind_from_raw < 'env > (local_ref : jobject) -> Self :: Kind < 'env > { T :: kind_from_raw (local_ref) } unsafe fn global_kind_from_raw (global_ref : jobject) -> Self :: GlobalKind { T :: global_kind_from_raw (global_ref) } }
};
}
