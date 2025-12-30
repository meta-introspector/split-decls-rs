// Generated macro for impl_1165 (impl)
macro_rules! Depcrate_scaffold_names_storageimpl_1165 {
() => {
// Module: crate::scaffold::names_storage
// Provides: {"impl_1165"}
// Dependencies: {}
impl < M : DynamicDataMarker , Variables > OptionalNames < Variables , DataPayload < M > > where Variables : Copy , { # [inline] pub (crate) fn as_borrowed < 'a > (& 'a self ,) -> OptionalNames < Variables , & 'a < M :: DataStruct as Yokeable < 'a > > :: Output > { match self { Self :: None => OptionalNames :: None , Self :: SingleLength { variables , payload } => OptionalNames :: SingleLength { variables : * variables , payload : payload . get () , } , } } }
};
}
