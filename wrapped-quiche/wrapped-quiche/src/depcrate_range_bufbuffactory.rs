// Generated macro for BufFactory (trait)
macro_rules! Depcrate_range_bufBufFactory {
() => {
// Module: crate::range_buf
// Provides: {"BufFactory"}
// Dependencies: {}
# [doc = " A trait for providing internal storage buffers for `RangeBuf`."] # [doc = " The associated type `Buf` can be any type that dereferences to"] # [doc = " a slice, but should be fast to clone, eg. by wrapping it with an"] # [doc = " [`Arc`]."] pub trait BufFactory : Clone + Default + Debug { # [doc = " The type of the generated buffer."] type Buf : Clone + Debug + AsRef < [u8] > ; # [doc = " Generate a new buffer from a given slice, the buffer must contain the"] # [doc = " same data as the original slice."] fn buf_from_slice (buf : & [u8]) -> Self :: Buf ; }
};
}
