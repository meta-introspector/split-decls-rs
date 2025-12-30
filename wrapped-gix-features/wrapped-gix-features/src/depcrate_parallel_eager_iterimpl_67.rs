// Generated macro for impl_67 (impl)
macro_rules! Depcrate_parallel_eager_iterimpl_67 {
() => {
// Module: crate::parallel::eager_iter
// Provides: {"impl_67"}
// Dependencies: {}
impl < I > Iterator for EagerIterIf < I > where I : Iterator + Send + 'static , < I as Iterator > :: Item : Send , { type Item = I :: Item ; fn next (& mut self) -> Option < Self :: Item > { match self { EagerIterIf :: OnDemand (i) => i . next () , EagerIterIf :: Eager (i) => i . next () , } } fn size_hint (& self) -> (usize , Option < usize >) { match self { EagerIterIf :: OnDemand (i) => i . size_hint () , EagerIterIf :: Eager (i) => i . size_hint () , } } }
};
}
