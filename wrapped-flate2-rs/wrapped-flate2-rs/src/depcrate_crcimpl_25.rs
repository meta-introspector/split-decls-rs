// Generated macro for impl_25 (impl)
macro_rules! Depcrate_crcimpl_25 {
() => {
// Module: crate::crc
// Provides: {"impl_25"}
// Dependencies: {}
impl < R : Read > CrcReader < R > { # [doc = " Create a new `CrcReader`."] pub fn new (r : R) -> CrcReader < R > { CrcReader { inner : r , crc : Crc :: new () , } } }
};
}
