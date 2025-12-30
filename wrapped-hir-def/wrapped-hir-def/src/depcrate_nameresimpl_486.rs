// Generated macro for impl_486 (impl)
macro_rules! Depcrate_nameresimpl_486 {
() => {
// Module: crate::nameres
// Provides: {"impl_486"}
// Dependencies: {}
impl LocalDefMap { pub (crate) const EMPTY : & Self = & Self { extern_prelude : FxIndexMap :: with_hasher (rustc_hash :: FxBuildHasher) } ; fn shrink_to_fit (& mut self) { let Self { extern_prelude } = self ; extern_prelude . shrink_to_fit () ; } pub (crate) fn extern_prelude (& self ,) -> impl DoubleEndedIterator < Item = (& Name , (CrateRootModuleId , Option < ExternCrateId >)) > + '_ { self . extern_prelude . iter () . map (| (name , & def) | (name , def)) } }
};
}
