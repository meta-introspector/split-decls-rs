// Generated macro for impl_323 (impl)
macro_rules! Depcrate_concurrent_stream_mapimpl_323 {
() => {
// Module: crate::concurrent_stream::map
// Provides: {"impl_323"}
// Dependencies: {}
impl < CS , F , FutT , T , FutB , B > ConcurrentStream for Map < CS , F , FutT , T , FutB , B > where CS : ConcurrentStream < Item = T , Future = FutT > , F : Fn (T) -> FutB , F : Clone , FutT : Future < Output = T > , FutB : Future < Output = B > , { type Future = MapFuture < F , FutT , T , FutB , B > ; type Item = B ; async fn drive < C > (self , consumer : C) -> C :: Output where C : Consumer < Self :: Item , Self :: Future > , { let consumer = MapConsumer { inner : consumer , f : self . f , _phantom : PhantomData , } ; self . inner . drive (consumer) . await } fn concurrency_limit (& self) -> Option < NonZeroUsize > { self . inner . concurrency_limit () } fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } }
};
}
