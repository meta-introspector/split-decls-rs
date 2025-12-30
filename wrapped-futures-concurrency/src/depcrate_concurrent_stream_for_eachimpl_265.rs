// Generated macro for impl_265 (impl)
macro_rules! Depcrate_concurrent_stream_for_eachimpl_265 {
() => {
// Module: crate::concurrent_stream::for_each
// Provides: {"impl_265"}
// Dependencies: {}
impl < A , T , F , B > ForEachConsumer < A , T , F , B > where A : Future < Output = T > , F : Fn (T) -> B , B : Future < Output = () > , { pub (crate) fn new (limit : Option < NonZeroUsize > , f : F) -> Self { let limit = match limit { Some (n) => n . get () , None => usize :: MAX , } ; Self { limit , f , _phantom : PhantomData , count : Arc :: new (AtomicUsize :: new (0)) , group : FuturesUnordered :: new () , } } }
};
}
