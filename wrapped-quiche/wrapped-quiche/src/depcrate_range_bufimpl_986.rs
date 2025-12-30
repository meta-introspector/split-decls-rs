// Generated macro for impl_986 (impl)
macro_rules! Depcrate_range_bufimpl_986 {
() => {
// Module: crate::range_buf
// Provides: {"impl_986"}
// Dependencies: {}
impl < F : BufFactory > PartialOrd for RangeBuf < F > { fn partial_cmp (& self , other : & RangeBuf < F >) -> Option < cmp :: Ordering > { Some (self . cmp (other)) } }
};
}
