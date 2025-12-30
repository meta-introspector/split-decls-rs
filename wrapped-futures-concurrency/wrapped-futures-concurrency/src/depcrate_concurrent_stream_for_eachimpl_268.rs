// Generated macro for impl_268 (impl)
macro_rules! Depcrate_concurrent_stream_for_eachimpl_268 {
() => {
// Module: crate::concurrent_stream::for_each
// Provides: {"impl_268"}
// Dependencies: {}
impl < F , FutT , T , FutB > ForEachFut < F , FutT , T , FutB > where FutT : Future < Output = T > , F : Fn (T) -> FutB , FutB : Future < Output = () > , { fn new (f : F , fut_t : FutT , count : Arc < AtomicUsize >) -> Self { Self { done : false , count , f , fut_t : Some (fut_t) , fut_b : None , } } }
};
}
