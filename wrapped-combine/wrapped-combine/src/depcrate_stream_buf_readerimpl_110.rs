// Generated macro for impl_110 (impl)
macro_rules! Depcrate_stream_buf_readerimpl_110 {
() => {
// Module: crate::stream::buf_reader
// Provides: {"impl_110"}
// Dependencies: {}
impl < R > CombineBuffer < R > for Buffer { fn buffer < 'a > (& 'a self , _read : & 'a R) -> & 'a [u8] { & self . 0 } fn advance (& mut self , _read : & mut R , len : usize) { self . 0 . advance (len) ; } # [cfg (feature = "pin-project-lite")] fn advance_pin (& mut self , _read : Pin < & mut R > , len : usize) { self . 0 . advance (len) ; } }
};
}
