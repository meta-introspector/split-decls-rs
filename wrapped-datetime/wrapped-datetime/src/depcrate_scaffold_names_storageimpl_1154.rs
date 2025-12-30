// Generated macro for impl_1154 (impl)
macro_rules! Depcrate_scaffold_names_storageimpl_1154 {
() => {
// Module: crate::scaffold::names_storage
// Provides: {"impl_1154"}
// Dependencies: {}
impl < M : DynamicDataMarker , Variables > Clone for DataPayloadWithVariables < M , Variables > where Variables : Clone , DataPayload < M > : Clone , { fn clone (& self) -> Self { Self { inner : self . inner . clone () , } } }
};
}
