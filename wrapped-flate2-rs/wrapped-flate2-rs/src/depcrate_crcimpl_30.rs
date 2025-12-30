// Generated macro for impl_30 (impl)
macro_rules! Depcrate_crcimpl_30 {
() => {
// Module: crate::crc
// Provides: {"impl_30"}
// Dependencies: {}
impl < W > CrcWriter < W > { # [doc = " Get the Crc for this `CrcWriter`."] pub fn crc (& self) -> & Crc { & self . crc } # [doc = " Get the writer that is wrapped by this `CrcWriter`."] pub fn into_inner (self) -> W { self . inner } # [doc = " Get the writer that is wrapped by this `CrcWriter` by reference."] pub fn get_ref (& self) -> & W { & self . inner } # [doc = " Get a mutable reference to the writer that is wrapped by this `CrcWriter`."] pub fn get_mut (& mut self) -> & mut W { & mut self . inner } # [doc = " Reset the Crc in this `CrcWriter`."] pub fn reset (& mut self) { self . crc . reset () ; } }
};
}
