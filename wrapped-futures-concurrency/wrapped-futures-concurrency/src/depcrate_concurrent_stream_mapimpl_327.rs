// Generated macro for impl_327 (impl)
macro_rules! Depcrate_concurrent_stream_mapimpl_327 {
() => {
// Module: crate::concurrent_stream::map
// Provides: {"impl_327"}
// Dependencies: {}
impl < F , FutT , T , FutB , B > MapFuture < F , FutT , T , FutB , B > where FutT : Future < Output = T > , F : Fn (T) -> FutB , FutB : Future < Output = B > , { fn new (f : F , fut_t : FutT) -> Self { Self { done : false , f , fut_t : Some (fut_t) , fut_b : None , } } }
};
}
