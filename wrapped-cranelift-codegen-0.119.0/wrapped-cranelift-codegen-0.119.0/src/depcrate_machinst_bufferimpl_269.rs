// Generated macro for impl_269 (impl)
macro_rules! Depcrate_machinst_bufferimpl_269 {
() => {
// Module: crate::machinst::buffer
// Provides: {"impl_269"}
// Dependencies: {}
impl PatchRegion { # [doc = " Consume the patch region to yield a mutable slice of the [`MachBuffer`] data buffer."] pub fn patch < I : VCodeInst > (self , buffer : & mut MachBuffer < I >) -> & mut [u8] { & mut buffer . data [self . range] } }
};
}
