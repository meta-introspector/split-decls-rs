// Generated macro for impl_237 (impl)
macro_rules! Depcrate_rayon_setimpl_237 {
() => {
// Module: crate::rayon::set
// Provides: {"impl_237"}
// Dependencies: {}
impl < 'a , T , S > ParallelDrainRange < usize > for & 'a mut IndexSet < T , S > where T : Send , { type Item = T ; type Iter = ParDrain < 'a , T > ; fn par_drain < R : RangeBounds < usize > > (self , range : R) -> Self :: Iter { ParDrain { entries : self . map . core . par_drain (range) , } } }
};
}
