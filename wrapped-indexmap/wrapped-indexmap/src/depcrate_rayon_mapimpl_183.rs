// Generated macro for impl_183 (impl)
macro_rules! Depcrate_rayon_mapimpl_183 {
() => {
// Module: crate::rayon::map
// Provides: {"impl_183"}
// Dependencies: {}
impl < 'a , K , V , S > ParallelDrainRange < usize > for & 'a mut IndexMap < K , V , S > where K : Send , V : Send , { type Item = (K , V) ; type Iter = ParDrain < 'a , K , V > ; fn par_drain < R : RangeBounds < usize > > (self , range : R) -> Self :: Iter { ParDrain { entries : self . core . par_drain (range) , } } }
};
}
