// Generated macro for impl_985 (impl)
macro_rules! Depcrate_range_bufimpl_985 {
() => {
// Module: crate::range_buf
// Provides: {"impl_985"}
// Dependencies: {}
impl < F : BufFactory > Ord for RangeBuf < F > { fn cmp (& self , other : & RangeBuf < F >) -> cmp :: Ordering { self . off . cmp (& other . off) . reverse () } }
};
}
