// Generated macro for impl_280 (impl)
macro_rules! Depcrate_concurrent_stream_from_concurrent_streamimpl_280 {
() => {
// Module: crate::concurrent_stream::from_concurrent_stream
// Provides: {"impl_280"}
// Dependencies: {}
impl < T > FromConcurrentStream < T > for Vec < T > { async fn from_concurrent_stream < S > (iter : S) -> Self where S : IntoConcurrentStream < Item = T > , { let stream = iter . into_co_stream () ; let mut output = Vec :: with_capacity (stream . size_hint () . 1 . unwrap_or_default ()) ; stream . drive (VecConsumer :: new (& mut output)) . await ; output } }
};
}
