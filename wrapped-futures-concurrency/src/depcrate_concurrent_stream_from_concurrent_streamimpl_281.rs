// Generated macro for impl_281 (impl)
macro_rules! Depcrate_concurrent_stream_from_concurrent_streamimpl_281 {
() => {
// Module: crate::concurrent_stream::from_concurrent_stream
// Provides: {"impl_281"}
// Dependencies: {}
impl < T , E > FromConcurrentStream < Result < T , E > > for Result < Vec < T > , E > { async fn from_concurrent_stream < S > (iter : S) -> Self where S : IntoConcurrentStream < Item = Result < T , E > > , { let stream = iter . into_co_stream () ; let mut output = Ok (Vec :: with_capacity (stream . size_hint () . 1 . unwrap_or_default ())) ; stream . drive (ResultVecConsumer :: new (& mut output)) . await ; output } }
};
}
