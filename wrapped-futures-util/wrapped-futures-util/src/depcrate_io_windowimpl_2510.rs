// Generated macro for impl_2510 (impl)
macro_rules! Depcrate_io_windowimpl_2510 {
() => {
// Module: crate::io::window
// Provides: {"impl_2510"}
// Dependencies: {}
impl < T : AsMut < [u8] > > AsMut < [u8] > for Window < T > { fn as_mut (& mut self) -> & mut [u8] { & mut self . inner . as_mut () [self . range . start .. self . range . end] } }
};
}
