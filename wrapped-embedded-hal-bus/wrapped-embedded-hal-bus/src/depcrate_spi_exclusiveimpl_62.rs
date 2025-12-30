// Generated macro for impl_62 (impl)
macro_rules! Depcrate_spi_exclusiveimpl_62 {
() => {
// Module: crate::spi::exclusive
// Provides: {"impl_62"}
// Dependencies: {}
impl < BUS , CS , D > ErrorType for ExclusiveDevice < BUS , CS , D > where BUS : ErrorType , CS : OutputPin , { type Error = DeviceError < BUS :: Error , CS :: Error > ; }
};
}
