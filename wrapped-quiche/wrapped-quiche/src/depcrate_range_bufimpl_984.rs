// Generated macro for impl_984 (impl)
macro_rules! Depcrate_range_bufimpl_984 {
() => {
// Module: crate::range_buf
// Provides: {"impl_984"}
// Dependencies: {}
impl < F : BufFactory > Deref for RangeBuf < F > { type Target = [u8] ; fn deref (& self) -> & [u8] { & self . data . as_ref () [self . pos .. self . start + self . len] } }
};
}
