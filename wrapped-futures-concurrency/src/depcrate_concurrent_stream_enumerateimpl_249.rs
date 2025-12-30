// Generated macro for impl_249 (impl)
macro_rules! Depcrate_concurrent_stream_enumerateimpl_249 {
() => {
// Module: crate::concurrent_stream::enumerate
// Provides: {"impl_249"}
// Dependencies: {}
impl < FutT , T > EnumerateFuture < FutT , T > where FutT : Future < Output = T > , { fn new (fut_t : FutT , count : usize) -> Self { Self { done : false , fut_t , count , } } }
};
}
