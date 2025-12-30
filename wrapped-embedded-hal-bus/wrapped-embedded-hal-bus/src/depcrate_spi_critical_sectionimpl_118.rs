// Generated macro for impl_118 (impl)
macro_rules! Depcrate_spi_critical_sectionimpl_118 {
() => {
// Module: crate::spi::critical_section
// Provides: {"impl_118"}
// Dependencies: {}
impl < Word : Copy + 'static , BUS , CS , D > SpiDevice < Word > for CriticalSectionDevice < '_ , BUS , CS , D > where BUS : SpiBus < Word > , CS : OutputPin , D : DelayNs , { # [inline] fn transaction (& mut self , operations : & mut [Operation < '_ , Word >]) -> Result < () , Self :: Error > { critical_section :: with (| cs | { let bus = & mut * self . bus . borrow_ref_mut (cs) ; transaction (operations , bus , & mut self . delay , & mut self . cs) }) } }
};
}
