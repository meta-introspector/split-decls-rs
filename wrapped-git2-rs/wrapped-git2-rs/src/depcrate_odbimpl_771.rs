// Generated macro for impl_771 (impl)
macro_rules! Depcrate_odbimpl_771 {
() => {
// Module: crate::odb
// Provides: {"impl_771"}
// Dependencies: {}
impl < 'repo > io :: Read for OdbReader < 'repo > { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { unsafe { let ptr = buf . as_ptr () as * mut c_char ; let len = buf . len () ; let res = raw :: git_odb_stream_read (self . raw , ptr , len) ; if res < 0 { Err (io :: Error :: new (io :: ErrorKind :: Other , "Read error")) } else { Ok (res as _) } } } }
};
}
