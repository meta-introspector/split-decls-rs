// Generated macro for impl_26 (impl)
macro_rules! Depcrate_crcimpl_26 {
() => {
// Module: crate::crc
// Provides: {"impl_26"}
// Dependencies: {}
impl < R > CrcReader < R > { # [doc = " Get the Crc for this `CrcReader`."] pub fn crc (& self) -> & Crc { & self . crc } # [doc = " Get the reader that is wrapped by this `CrcReader`."] pub fn into_inner (self) -> R { self . inner } # [doc = " Get the reader that is wrapped by this `CrcReader` by reference."] pub fn get_ref (& self) -> & R { & self . inner } # [doc = " Get a mutable reference to the reader that is wrapped by this `CrcReader`."] pub fn get_mut (& mut self) -> & mut R { & mut self . inner } # [doc = " Reset the Crc in this `CrcReader`."] pub fn reset (& mut self) { self . crc . reset () ; } }
};
}
