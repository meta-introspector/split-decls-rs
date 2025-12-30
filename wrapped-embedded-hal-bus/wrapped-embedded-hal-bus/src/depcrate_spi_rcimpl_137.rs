// Generated macro for impl_137 (impl)
macro_rules! Depcrate_spi_rcimpl_137 {
() => {
// Module: crate::spi::rc
// Provides: {"impl_137"}
// Dependencies: {}
impl < Bus , Cs > RcDevice < Bus , Cs , super :: NoDelay > { # [doc = " Creates a new [`RcDevice`] without support for in-transaction delays."] # [doc = ""] # [doc = " **Warning**: It's advised to prefer [`RcDevice::new`],"] # [doc = " as the contract of [`SpiDevice`] requests support for in-transaction delays."] # [doc = ""] # [doc = " Refer to [`RefCellDevice::new_no_delay`](super::RefCellDevice::new_no_delay) for more information."] # [inline] pub fn new_no_delay (bus : Rc < RefCell < Bus > > , mut cs : Cs) -> Result < Self , Cs :: Error > where Cs : OutputPin , { cs . set_high () ? ; Ok (Self { bus , cs , delay : super :: NoDelay , }) } }
};
}
