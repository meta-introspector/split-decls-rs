// Generated macro for DataPayloadWithVariablesBorrowed (struct)
macro_rules! Depcrate_scaffold_names_storageDataPayloadWithVariablesBorrowed {
() => {
// Module: crate::scaffold::names_storage
// Provides: {"DataPayloadWithVariablesBorrowed"}
// Dependencies: {}
# [doc = " Borrowed version of [`DataPayloadWithVariables`]."] # [allow (missing_docs)] pub struct DataPayloadWithVariablesBorrowed < 'data , M : DynamicDataMarker , Variables > { pub (crate) inner : OptionalNames < Variables , & 'data < M :: DataStruct as Yokeable < 'data > > :: Output > , }
};
}
