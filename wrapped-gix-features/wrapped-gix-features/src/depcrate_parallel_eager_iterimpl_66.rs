// Generated macro for impl_66 (impl)
macro_rules! Depcrate_parallel_eager_iterimpl_66 {
() => {
// Module: crate::parallel::eager_iter
// Provides: {"impl_66"}
// Dependencies: {}
impl < I > EagerIterIf < I > where I : Iterator + Send + 'static , < I as Iterator > :: Item : Send , { # [doc = " Return a new `EagerIterIf` if `condition()` returns true."] # [doc = ""] # [doc = " For all other parameters, please see [`EagerIter::new()`]."] pub fn new (condition : impl FnOnce () -> bool , iter : I , chunk_size : usize , chunks_in_flight : usize) -> Self { if condition () { EagerIterIf :: Eager (EagerIter :: new (iter , chunk_size , chunks_in_flight)) } else { EagerIterIf :: OnDemand (iter) } } }
};
}
