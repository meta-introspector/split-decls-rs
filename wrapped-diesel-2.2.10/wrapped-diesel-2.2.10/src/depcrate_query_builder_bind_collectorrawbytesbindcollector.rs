// Generated macro for RawBytesBindCollector (struct)
macro_rules! Depcrate_query_builder_bind_collectorRawBytesBindCollector {
() => {
// Module: crate::query_builder::bind_collector
// Provides: {"RawBytesBindCollector"}
// Dependencies: {}
# [derive (Debug)] # [doc = " A bind collector used by backends which transmit bind parameters as an"] # [doc = " opaque blob of bytes."] # [doc = ""] # [doc = " For most backends, this is the concrete implementation of `BindCollector`"] # [doc = " that should be used."] # [diesel_derives :: __diesel_public_if (feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes" , public_fields (metadata , binds))] pub struct RawBytesBindCollector < DB : Backend + TypeMetadata > { # [doc = " The metadata associated with each bind parameter."] # [doc = ""] # [doc = " This vec is guaranteed to be the same length as `binds`."] pub (crate) metadata : Vec < DB :: TypeMetadata > , # [doc = " The serialized bytes for each bind parameter."] # [doc = ""] # [doc = " This vec is guaranteed to be the same length as `metadata`."] pub (crate) binds : Vec < Option < Vec < u8 > > > , }
};
}
