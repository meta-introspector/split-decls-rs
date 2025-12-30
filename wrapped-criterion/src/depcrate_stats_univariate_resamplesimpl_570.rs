// Generated macro for impl_570 (impl)
macro_rules! Depcrate_stats_univariate_resamplesimpl_570 {
() => {
// Module: crate::stats::univariate::resamples
// Provides: {"impl_570"}
// Dependencies: {}
# [allow (clippy :: should_implement_trait)] impl < 'a , A > Resamples < 'a , A > where A : 'a + Float , { pub fn new (sample : & 'a Sample < A >) -> Resamples < 'a , A > { let slice = sample ; Resamples { rng : new_rng () , sample : slice , stage : None , } } pub fn next (& mut self) -> & Sample < A > { let n = self . sample . len () ; let rng = & mut self . rng ; match self . stage { None => { let mut stage = Vec :: with_capacity (n) ; for _ in 0 .. n { let idx = rng . rand_range (0u64 .. (self . sample . len () as u64)) ; stage . push (self . sample [idx as usize]) ; } self . stage = Some (stage) ; } Some (ref mut stage) => { for elem in stage . iter_mut () { let idx = rng . rand_range (0u64 .. (self . sample . len () as u64)) ; * elem = self . sample [idx as usize] ; } } } if let Some (ref v) = self . stage { unsafe { mem :: transmute :: < & [A] , & Sample < A > > (v) } } else { unreachable ! () ; } } }
};
}
