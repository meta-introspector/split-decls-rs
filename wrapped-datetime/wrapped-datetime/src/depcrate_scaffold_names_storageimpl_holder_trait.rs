// Generated macro for impl_holder_trait (macro)
macro_rules! Depcrate_scaffold_names_storageimpl_holder_trait {
() => {
// Module: crate::scaffold::names_storage
// Provides: {"impl_holder_trait"}
// Dependencies: {}
macro_rules ! impl_holder_trait { ($ marker : path) => { impl UnstableSealed for $ marker { } impl < Variables > NamesContainer <$ marker , Variables > for $ marker where Variables : PartialEq + Copy + MaybeAsErrorField + fmt :: Debug , { type Container = DataPayloadWithVariables <$ marker , Variables >; } } ; }
};
}
