// Generated macro for impl_2432 (impl)
macro_rules! Depcrate_io_read_untilimpl_2432 {
() => {
// Module: crate::io::read_until
// Provides: {"impl_2432"}
// Dependencies: {}
impl < 'a , R : AsyncBufRead + ? Sized + Unpin > ReadUntil < 'a , R > { pub (super) fn new (reader : & 'a mut R , byte : u8 , buf : & 'a mut Vec < u8 >) -> Self { Self { reader , byte , buf , read : 0 } } }
};
}
