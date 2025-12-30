// Generated macro for impl_186 (impl)
macro_rules! Depcrate_writeimpl_186 {
() => {
// Module: crate::write
// Provides: {"impl_186"}
// Dependencies: {}
impl < 'a > SliceWrite < 'a > { # [doc = " Wraps a mutable slice so it can be used as a `Write`."] pub fn new (slice : & 'a mut [u8]) -> SliceWrite < 'a > { SliceWrite { slice , index : 0 } } # [doc = " Returns the number of bytes written to the underlying slice."] pub fn bytes_written (& self) -> usize { self . index } # [doc = " Returns the underlying slice."] pub fn into_inner (self) -> & 'a mut [u8] { self . slice } }
};
}
