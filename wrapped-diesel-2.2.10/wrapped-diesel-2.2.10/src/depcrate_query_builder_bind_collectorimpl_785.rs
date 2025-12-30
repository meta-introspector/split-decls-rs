// Generated macro for impl_785 (impl)
macro_rules! Depcrate_query_builder_bind_collectorimpl_785 {
() => {
// Module: crate::query_builder::bind_collector
// Provides: {"impl_785"}
// Dependencies: {}
# [allow (clippy :: new_without_default)] impl < DB : Backend + TypeMetadata > RawBytesBindCollector < DB > { # [doc = " Construct an empty `RawBytesBindCollector`"] pub fn new () -> Self { RawBytesBindCollector { metadata : Vec :: new () , binds : Vec :: new () , } } pub (crate) fn reborrow_buffer < 'a : 'b , 'b > (b : & 'b mut ByteWrapper < 'a >) -> ByteWrapper < 'b > { ByteWrapper (b . 0) } }
};
}
