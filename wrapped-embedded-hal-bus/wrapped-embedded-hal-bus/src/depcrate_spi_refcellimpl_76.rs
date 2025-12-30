// Generated macro for impl_76 (impl)
macro_rules! Depcrate_spi_refcellimpl_76 {
() => {
// Module: crate::spi::refcell
// Provides: {"impl_76"}
// Dependencies: {}
impl < BUS , CS , D > ErrorType for RefCellDevice < '_ , BUS , CS , D > where BUS : ErrorType , CS : OutputPin , { type Error = DeviceError < BUS :: Error , CS :: Error > ; }
};
}
