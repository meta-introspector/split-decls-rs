// Generated macro for impl_64 (impl)
macro_rules! Depcrate_buf_readerimpl_64 {
() => {
// Module: crate::buf::reader
// Provides: {"impl_64"}
// Dependencies: {}
impl < B : Buf + Sized > io :: Read for Reader < B > { fn read (& mut self , dst : & mut [u8]) -> io :: Result < usize > { let len = cmp :: min (self . buf . remaining () , dst . len ()) ; Buf :: copy_to_slice (& mut self . buf , & mut dst [0 .. len]) ; Ok (len) } }
};
}
