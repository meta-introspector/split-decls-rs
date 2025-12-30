// Generated macro for impl_192 (impl)
macro_rules! Depcrate_refs_castimpl_192 {
() => {
// Module: crate::refs::cast
// Provides: {"impl_192"}
// Dependencies: {}
unsafe impl < 'any , 'from , To : Reference > Reference for Cast < 'any , 'from , To > { type Kind < 'local > = To :: Kind < 'local > ; type GlobalKind = To :: GlobalKind ; fn as_raw (& self) -> jobject { self . to . as_raw () } fn class_name () -> Cow < 'static , JNIStr > { To :: class_name () } fn lookup_class < 'caller > (env : & Env < '_ > , loader_context : LoaderContext ,) -> crate :: errors :: Result < impl Deref < Target = Global < JClass < 'static > > > + 'caller > { To :: lookup_class (env , loader_context) } unsafe fn kind_from_raw < 'env > (local_ref : jobject) -> Self :: Kind < 'env > { To :: kind_from_raw (local_ref) } unsafe fn global_kind_from_raw (global_ref : jobject) -> Self :: GlobalKind { To :: global_kind_from_raw (global_ref) } }
};
}
