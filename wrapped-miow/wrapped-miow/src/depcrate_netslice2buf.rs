// Generated macro for slice2buf (function)
macro_rules! Depcrate_netslice2buf {
() => {
// Module: crate::net
// Provides: {"slice2buf"}
// Dependencies: {}
unsafe fn slice2buf (slice : & [u8]) -> WSABUF { WSABUF { len : cmp :: min (slice . len () , u32 :: MAX as usize) as u32 , buf : slice . as_ptr () as * mut _ , } }
};
}
