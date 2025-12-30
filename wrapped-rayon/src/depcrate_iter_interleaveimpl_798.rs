// Generated macro for impl_798 (impl)
macro_rules! Depcrate_iter_interleaveimpl_798 {
() => {
// Module: crate::iter::interleave
// Provides: {"impl_798"}
// Dependencies: {}
impl < I , J > InterleaveProducer < I , J > where I : Producer , J : Producer < Item = I :: Item > , { fn new (i : I , j : J , i_len : usize , j_len : usize , i_next : bool) -> InterleaveProducer < I , J > { InterleaveProducer { i , j , i_len , j_len , i_next , } } }
};
}
