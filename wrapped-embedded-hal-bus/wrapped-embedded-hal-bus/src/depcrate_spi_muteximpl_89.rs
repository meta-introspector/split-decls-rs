// Generated macro for impl_89 (impl)
macro_rules! Depcrate_spi_muteximpl_89 {
() => {
// Module: crate::spi::mutex
// Provides: {"impl_89"}
// Dependencies: {}
impl < BUS , CS , D > ErrorType for MutexDevice < '_ , BUS , CS , D > where BUS : ErrorType , CS : OutputPin , { type Error = DeviceError < BUS :: Error , CS :: Error > ; }
};
}
