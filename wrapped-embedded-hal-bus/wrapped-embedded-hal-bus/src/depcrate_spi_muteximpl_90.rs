// Generated macro for impl_90 (impl)
macro_rules! Depcrate_spi_muteximpl_90 {
() => {
// Module: crate::spi::mutex
// Provides: {"impl_90"}
// Dependencies: {}
impl < Word : Copy + 'static , BUS , CS , D > SpiDevice < Word > for MutexDevice < '_ , BUS , CS , D > where BUS : SpiBus < Word > , CS : OutputPin , D : DelayNs , { # [inline] fn transaction (& mut self , operations : & mut [Operation < '_ , Word >]) -> Result < () , Self :: Error > { let bus = & mut * self . bus . lock () . unwrap () ; transaction (operations , bus , & mut self . delay , & mut self . cs) } }
};
}
