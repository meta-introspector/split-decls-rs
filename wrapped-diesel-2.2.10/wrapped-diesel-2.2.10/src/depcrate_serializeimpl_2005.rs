// Generated macro for impl_2005 (impl)
macro_rules! Depcrate_serializeimpl_2005 {
() => {
// Module: crate::serialize
// Provides: {"impl_2005"}
// Dependencies: {}
# [cfg (test)] impl < 'a , DB : Backend > Output < 'a , 'static , DB > { # [doc = " Returns a `Output` suitable for testing `ToSql` implementations."] # [doc = " Unsafe to use for testing types which perform dynamic metadata lookup."] pub fn test (buffer : < DB :: BindCollector < 'a > as BindCollector < 'a , DB > > :: Buffer) -> Self { Self { out : buffer , metadata_lookup : None , } } }
};
}
