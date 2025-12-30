// Generated macro for impl_64 (impl)
macro_rules! Depcrate_valueimpl_64 {
() => {
// Module: crate::value
// Provides: {"impl_64"}
// Dependencies: {}
impl < T , const N : usize > From < BufMut < T , N > > for Buf < T , N > { fn from (value : BufMut < T , N >) -> Self { # [cfg (feature = "alloc")] { Buf { inner : value . inner . into_boxed_slice () , } } # [cfg (not (feature = "alloc"))] { Buf { inner : value . inner } } } }
};
}
