// Generated macro for impl_139 (impl)
macro_rules! Depcrate_spi_rcimpl_139 {
() => {
// Module: crate::spi::rc
// Provides: {"impl_139"}
// Dependencies: {}
impl < Word , Bus , Cs , Delay > SpiDevice < Word > for RcDevice < Bus , Cs , Delay > where Word : Copy + 'static , Bus : SpiBus < Word > , Cs : OutputPin , Delay : DelayNs , { # [inline] fn transaction (& mut self , operations : & mut [Operation < '_ , Word >]) -> Result < () , Self :: Error > { let bus = & mut * self . bus . borrow_mut () ; transaction (operations , bus , & mut self . delay , & mut self . cs) } }
};
}
