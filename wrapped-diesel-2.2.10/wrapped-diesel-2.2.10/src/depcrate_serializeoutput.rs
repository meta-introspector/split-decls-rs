// Generated macro for Output (struct)
macro_rules! Depcrate_serializeOutput {
() => {
// Module: crate::serialize
// Provides: {"Output"}
// Dependencies: {}
# [doc = " Wraps a buffer to be written by `ToSql` with additional backend specific"] # [doc = " utilities."] pub struct Output < 'a , 'b , DB > where DB : Backend , DB :: MetadataLookup : 'a , { out : < DB :: BindCollector < 'a > as BindCollector < 'a , DB > > :: Buffer , metadata_lookup : Option < & 'b mut DB :: MetadataLookup > , }
};
}
