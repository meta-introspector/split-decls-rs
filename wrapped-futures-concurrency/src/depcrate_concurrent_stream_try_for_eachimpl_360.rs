// Generated macro for impl_360 (impl)
macro_rules! Depcrate_concurrent_stream_try_for_eachimpl_360 {
() => {
// Module: crate::concurrent_stream::try_for_each
// Provides: {"impl_360"}
// Dependencies: {}
impl < F , FutT , T , FutB , B > TryForEachFut < F , FutT , T , FutB , B > where FutT : Future < Output = T > , F : Clone + Fn (T) -> FutB , FutB : Future < Output = B > , B : Try < Output = () > , { fn new (f : F , fut_t : FutT , count : Arc < AtomicUsize >) -> Self { Self { done : false , count , f , fut_t : Some (fut_t) , fut_b : None , } } }
};
}
