// Generated macro for impl_111 (impl)
macro_rules! Depcrate_collections_hash_setimpl_111 {
() => {
// Module: crate::collections::hash_set
// Provides: {"impl_111"}
// Dependencies: {}
impl < 'a , T : Send , S > ParallelDrainFull for & 'a mut HashSet < T , S > { type Iter = Drain < 'a , T > ; type Item = T ; fn par_drain (self) -> Self :: Iter { let vec : Vec < _ > = self . drain () . collect () ; Drain { inner : vec . into_par_iter () , marker : PhantomData , } } }
};
}
