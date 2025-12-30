// Generated macro for impl_117 (impl)
macro_rules! Depcrate_spi_critical_sectionimpl_117 {
() => {
// Module: crate::spi::critical_section
// Provides: {"impl_117"}
// Dependencies: {}
impl < BUS , CS , D > ErrorType for CriticalSectionDevice < '_ , BUS , CS , D > where BUS : ErrorType , CS : OutputPin , { type Error = DeviceError < BUS :: Error , CS :: Error > ; }
};
}
