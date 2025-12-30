// Generated macro for impl_1231 (impl)
macro_rules! Depcrate_sslimpl_1231 {
() => {
// Module: crate::ssl
// Provides: {"impl_1231"}
// Dependencies: {}
impl < S : Read + Write > Read for SslStream < S > { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { unsafe { self . read_uninit (util :: from_raw_parts_mut (buf . as_mut_ptr () . cast :: < MaybeUninit < u8 > > () , buf . len () ,)) } } }
};
}
