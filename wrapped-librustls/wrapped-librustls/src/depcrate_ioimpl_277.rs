// Generated macro for impl_277 (impl)
macro_rules! Depcrate_ioimpl_277 {
() => {
// Module: crate::io
// Provides: {"impl_277"}
// Dependencies: {}
impl Write for VectoredCallbackWriter { fn write (& mut self , buf : & [u8]) -> Result < usize > { self . write_vectored (& [IoSlice :: new (buf)]) } fn flush (& mut self) -> Result < () > { Ok (()) } fn write_vectored (& mut self , bufs : & [IoSlice < '_ >]) -> Result < usize > { let mut out_n = 0 ; let cb = self . callback ; let result = unsafe { cb (self . userdata , bufs . as_ptr () as * const rustls_iovec , bufs . len () , & mut out_n ,) } ; match result . 0 { 0 => Ok (out_n) , e => Err (Error :: from_raw_os_error (e)) , } } }
};
}
