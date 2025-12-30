// Generated macro for impl_77 (impl)
macro_rules! Depcrate_spi_refcellimpl_77 {
() => {
// Module: crate::spi::refcell
// Provides: {"impl_77"}
// Dependencies: {}
impl < Word : Copy + 'static , BUS , CS , D > SpiDevice < Word > for RefCellDevice < '_ , BUS , CS , D > where BUS : SpiBus < Word > , CS : OutputPin , D : DelayNs , { # [inline] fn transaction (& mut self , operations : & mut [Operation < '_ , Word >]) -> Result < () , Self :: Error > { let bus = & mut * self . bus . borrow_mut () ; transaction (operations , bus , & mut self . delay , & mut self . cs) } }
};
}
