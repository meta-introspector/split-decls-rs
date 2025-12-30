// Generated macro for impl_104 (impl)
macro_rules! Depcrate_spi_atomicimpl_104 {
() => {
// Module: crate::spi::atomic
// Provides: {"impl_104"}
// Dependencies: {}
impl < BUS , CS , D > ErrorType for AtomicDevice < '_ , BUS , CS , D > where BUS : ErrorType , CS : OutputPin , { type Error = AtomicError < DeviceError < BUS :: Error , CS :: Error > > ; }
};
}
