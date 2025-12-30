// Generated macro for impl_13 (impl)
macro_rules! Depcrate_readerimpl_13 {
() => {
// Module: crate::reader
// Provides: {"impl_13"}
// Dependencies: {}
impl < D : Digest , R : io :: Read > io :: Read for HashReader < D , R > { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { let bytes = self . reader . read (buf) ? ; if bytes > 0 { self . hasher . update (& buf [0 .. bytes]) ; } Ok (bytes) } }
};
}
