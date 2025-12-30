// Generated macro for impl_33 (impl)
macro_rules! Depcrateimpl_33 {
() => {
// Module: crate
// Provides: {"impl_33"}
// Dependencies: {}
impl < T : Default + Reuse > Drop for Pooled < T > { fn drop (& mut self) { let QueueShard { queue , elem_cnt , trim , max , } = self . pool ; if self . inner . reuse (* trim) { if elem_cnt . fetch_add (1 , Ordering :: Acquire) < * max { queue . push (std :: mem :: take (& mut self . inner)) ; return ; } elem_cnt . fetch_sub (1 , Ordering :: Release) ; } } }
};
}
