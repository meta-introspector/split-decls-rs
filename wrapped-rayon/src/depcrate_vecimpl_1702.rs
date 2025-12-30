// Generated macro for impl_1702 (impl)
macro_rules! Depcrate_vecimpl_1702 {
() => {
// Module: crate::vec
// Provides: {"impl_1702"}
// Dependencies: {}
impl < 'data , T : Send > ParallelDrainRange < usize > for & 'data mut Vec < T > { type Iter = Drain < 'data , T > ; type Item = T ; fn par_drain < R : RangeBounds < usize > > (self , range : R) -> Self :: Iter { Drain { orig_len : self . len () , range : simplify_range (range , self . len ()) , vec : self , } } }
};
}
