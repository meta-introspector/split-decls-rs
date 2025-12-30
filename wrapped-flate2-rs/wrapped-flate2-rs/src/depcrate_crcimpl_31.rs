// Generated macro for impl_31 (impl)
macro_rules! Depcrate_crcimpl_31 {
() => {
// Module: crate::crc
// Provides: {"impl_31"}
// Dependencies: {}
impl < W : Write > CrcWriter < W > { # [doc = " Create a new `CrcWriter`."] pub fn new (w : W) -> CrcWriter < W > { CrcWriter { inner : w , crc : Crc :: new () , } } }
};
}
