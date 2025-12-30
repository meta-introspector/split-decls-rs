// Generated macro for impl_177 (impl)
macro_rules! Depcrate_writeimpl_177 {
() => {
// Module: crate::write
// Provides: {"impl_177"}
// Dependencies: {}
# [cfg (feature = "std")] impl < W : io :: Write > Write for IoWrite < W > { type Error = io :: Error ; fn write_all (& mut self , buf : & [u8]) -> Result < () , Self :: Error > { self . 0 . write_all (buf) } }
};
}
