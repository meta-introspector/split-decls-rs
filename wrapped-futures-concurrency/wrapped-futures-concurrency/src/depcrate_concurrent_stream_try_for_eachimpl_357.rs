// Generated macro for impl_357 (impl)
macro_rules! Depcrate_concurrent_stream_try_for_eachimpl_357 {
() => {
// Module: crate::concurrent_stream::try_for_each
// Provides: {"impl_357"}
// Dependencies: {}
impl < FutT , T , F , FutB , B > TryForEachConsumer < FutT , T , F , FutB , B > where FutT : Future < Output = T > , F : Clone + Fn (T) -> FutB , FutB : Future < Output = B > , B : Try < Output = () > , { pub (crate) fn new (limit : Option < NonZeroUsize > , f : F) -> Self { let limit = match limit { Some (n) => n . get () , None => usize :: MAX , } ; Self { limit , f , residual : None , count : Arc :: new (AtomicUsize :: new (0)) , group : FuturesUnordered :: new () , _phantom : PhantomData , } } }
};
}
