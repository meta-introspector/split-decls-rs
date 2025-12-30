// Generated macro for impl_138 (impl)
macro_rules! Depcrate_spi_rcimpl_138 {
() => {
// Module: crate::spi::rc
// Provides: {"impl_138"}
// Dependencies: {}
impl < Bus , Cs , Delay > ErrorType for RcDevice < Bus , Cs , Delay > where Bus : ErrorType , Cs : OutputPin , { type Error = DeviceError < Bus :: Error , Cs :: Error > ; }
};
}
