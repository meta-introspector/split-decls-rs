// Generated macro for impl_125 (impl)
macro_rules! Depcrate_stream_buf_readerimpl_125 {
() => {
// Module: crate::stream::buf_reader
// Provides: {"impl_125"}
// Dependencies: {}
# [cfg (feature = "tokio-02")] impl bytes_05 :: BufMut for Bytes05 < '_ > { fn remaining_mut (& self) -> usize { self . 0 . remaining_mut () } unsafe fn advance_mut (& mut self , cnt : usize) { self . 0 . advance_mut (cnt) } fn bytes_mut (& mut self) -> & mut [MaybeUninit < u8 >] { unsafe { & mut * (self . 0 . chunk_mut () as * mut _ as * mut [MaybeUninit < u8 >]) } } }
};
}
