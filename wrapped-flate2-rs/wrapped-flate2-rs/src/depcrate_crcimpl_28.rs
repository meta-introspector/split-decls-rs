// Generated macro for impl_28 (impl)
macro_rules! Depcrate_crcimpl_28 {
() => {
// Module: crate::crc
// Provides: {"impl_28"}
// Dependencies: {}
impl < R : BufRead > BufRead for CrcReader < R > { fn fill_buf (& mut self) -> io :: Result < & [u8] > { self . inner . fill_buf () } fn consume (& mut self , amt : usize) { if let Ok (data) = self . inner . fill_buf () { self . crc . update (& data [.. amt]) ; } self . inner . consume (amt) ; } }
};
}
