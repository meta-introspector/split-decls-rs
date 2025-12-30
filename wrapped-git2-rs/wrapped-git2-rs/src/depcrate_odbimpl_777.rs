// Generated macro for impl_777 (impl)
macro_rules! Depcrate_odbimpl_777 {
() => {
// Module: crate::odb
// Provides: {"impl_777"}
// Dependencies: {}
impl < 'repo > io :: Write for OdbWriter < 'repo > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { unsafe { let ptr = buf . as_ptr () as * const c_char ; let len = buf . len () ; let res = raw :: git_odb_stream_write (self . raw , ptr , len) ; if res < 0 { Err (io :: Error :: new (io :: ErrorKind :: Other , "Write error")) } else { Ok (buf . len ()) } } } fn flush (& mut self) -> io :: Result < () > { Ok (()) } }
};
}
