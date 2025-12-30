// Generated macro for impl_202 (impl)
macro_rules! Depcrate_refs_referenceimpl_202 {
() => {
// Module: crate::refs::reference
// Provides: {"impl_202"}
// Dependencies: {}
unsafe impl < T > Reference for & T where T : Reference , { type Kind < 'local > = T :: Kind < 'local > ; type GlobalKind = T :: GlobalKind ; fn as_raw (& self) -> jobject { (* self) . as_raw () } fn class_name () -> Cow < 'static , JNIStr > { T :: class_name () } fn lookup_class < 'caller > (env : & Env < '_ > , loader_context : LoaderContext ,) -> crate :: errors :: Result < impl Deref < Target = Global < JClass < 'static > > > + 'caller > { T :: lookup_class (env , loader_context) } unsafe fn kind_from_raw < 'env > (local_ref : jobject) -> Self :: Kind < 'env > { T :: kind_from_raw (local_ref) } unsafe fn global_kind_from_raw (global_ref : jobject) -> Self :: GlobalKind { T :: global_kind_from_raw (global_ref) } }
};
}
