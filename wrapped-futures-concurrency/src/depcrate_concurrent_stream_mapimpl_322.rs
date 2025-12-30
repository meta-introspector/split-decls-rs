// Generated macro for impl_322 (impl)
macro_rules! Depcrate_concurrent_stream_mapimpl_322 {
() => {
// Module: crate::concurrent_stream::map
// Provides: {"impl_322"}
// Dependencies: {}
impl < CS , F , FutT , T , FutB , B > Map < CS , F , FutT , T , FutB , B > where CS : ConcurrentStream < Item = T , Future = FutT > , F : Fn (T) -> FutB , F : Clone , FutT : Future < Output = T > , FutB : Future < Output = B > , { pub (crate) fn new (inner : CS , f : F) -> Self { Self { inner , f , _phantom : PhantomData , } } }
};
}
