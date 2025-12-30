// Generated macro for impl_27 (impl)
macro_rules! Depcrate_crcimpl_27 {
() => {
// Module: crate::crc
// Provides: {"impl_27"}
// Dependencies: {}
impl < R : Read > Read for CrcReader < R > { fn read (& mut self , into : & mut [u8]) -> io :: Result < usize > { let amt = self . inner . read (into) ? ; self . crc . update (& into [.. amt]) ; Ok (amt) } }
};
}
