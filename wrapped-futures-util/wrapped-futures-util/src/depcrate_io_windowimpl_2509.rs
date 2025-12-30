// Generated macro for impl_2509 (impl)
macro_rules! Depcrate_io_windowimpl_2509 {
() => {
// Module: crate::io::window
// Provides: {"impl_2509"}
// Dependencies: {}
impl < T : AsRef < [u8] > > AsRef < [u8] > for Window < T > { fn as_ref (& self) -> & [u8] { & self . inner . as_ref () [self . range . start .. self . range . end] } }
};
}
