// Generated macro for impl_95 (impl)
macro_rules! Depcrate_collections_hash_mapimpl_95 {
() => {
// Module: crate::collections::hash_map
// Provides: {"impl_95"}
// Dependencies: {}
impl < 'a , K : Send , V : Send , S > ParallelDrainFull for & 'a mut HashMap < K , V , S > { type Iter = Drain < 'a , K , V > ; type Item = (K , V) ; fn par_drain (self) -> Self :: Iter { let vec : Vec < _ > = self . drain () . collect () ; Drain { inner : vec . into_par_iter () , marker : PhantomData , } } }
};
}
