// Generated macro for impl_96 (impl)
macro_rules! Depcrate_buf_writerimpl_96 {
() => {
// Module: crate::buf::writer
// Provides: {"impl_96"}
// Dependencies: {}
impl < B : BufMut + Sized > io :: Write for Writer < B > { fn write (& mut self , src : & [u8]) -> io :: Result < usize > { let n = cmp :: min (self . buf . remaining_mut () , src . len ()) ; self . buf . put_slice (& src [.. n]) ; Ok (n) } fn flush (& mut self) -> io :: Result < () > { Ok (()) } }
};
}
