// Generated macro for impl_1156 (impl)
macro_rules! Depcrate_scaffold_names_storageimpl_1156 {
() => {
// Module: crate::scaffold::names_storage
// Provides: {"impl_1156"}
// Dependencies: {}
impl < M : DynamicDataMarker , Variables > fmt :: Debug for DataPayloadWithVariables < M , Variables > where Variables : fmt :: Debug , DataPayload < M > : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . inner . fmt (f) } }
};
}
