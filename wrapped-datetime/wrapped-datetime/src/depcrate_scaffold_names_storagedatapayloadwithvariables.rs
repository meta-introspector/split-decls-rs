// Generated macro for DataPayloadWithVariables (struct)
macro_rules! Depcrate_scaffold_names_storageDataPayloadWithVariables {
() => {
// Module: crate::scaffold::names_storage
// Provides: {"DataPayloadWithVariables"}
// Dependencies: {}
# [doc = " An implementation of [`MaybePayload`] that wraps an optional [`DataPayload`],"] # [doc = " parameterized by `Variables`."] pub struct DataPayloadWithVariables < M : DynamicDataMarker , Variables > { inner : OptionalNames < Variables , DataPayload < M > > , }
};
}
