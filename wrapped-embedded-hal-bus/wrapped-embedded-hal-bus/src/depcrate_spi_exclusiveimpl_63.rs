// Generated macro for impl_63 (impl)
macro_rules! Depcrate_spi_exclusiveimpl_63 {
() => {
// Module: crate::spi::exclusive
// Provides: {"impl_63"}
// Dependencies: {}
impl < Word : Copy + 'static , BUS , CS , D > SpiDevice < Word > for ExclusiveDevice < BUS , CS , D > where BUS : SpiBus < Word > , CS : OutputPin , D : DelayNs , { # [inline] fn transaction (& mut self , operations : & mut [Operation < '_ , Word >]) -> Result < () , Self :: Error > { transaction (operations , & mut self . bus , & mut self . delay , & mut self . cs) } }
};
}
