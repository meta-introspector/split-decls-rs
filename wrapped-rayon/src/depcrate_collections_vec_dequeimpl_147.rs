// Generated macro for impl_147 (impl)
macro_rules! Depcrate_collections_vec_dequeimpl_147 {
() => {
// Module: crate::collections::vec_deque
// Provides: {"impl_147"}
// Dependencies: {}
impl < 'a , T : Send > ParallelDrainRange < usize > for & 'a mut VecDeque < T > { type Iter = Drain < 'a , T > ; type Item = T ; fn par_drain < R : RangeBounds < usize > > (self , range : R) -> Self :: Iter { Drain { orig_len : self . len () , range : simplify_range (range , self . len ()) , deque : self , } } }
};
}
