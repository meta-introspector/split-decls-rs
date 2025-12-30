// Generated macro for impl_101 (impl)
macro_rules! Depcrate_spi_atomicimpl_101 {
() => {
// Module: crate::spi::atomic
// Provides: {"impl_101"}
// Dependencies: {}
impl < 'a , BUS , CS , D > AtomicDevice < 'a , BUS , CS , D > { # [doc = " Create a new [`AtomicDevice`]."] # [doc = ""] # [doc = " This sets the `cs` pin high, and returns an error if that fails. It is recommended"] # [doc = " to set the pin high the moment it's configured as an output, to avoid glitches."] # [inline] pub fn new (bus : & 'a AtomicCell < BUS > , mut cs : CS , delay : D) -> Result < Self , CS :: Error > where CS : OutputPin , { cs . set_high () ? ; Ok (Self { bus , cs , delay }) } }
};
}
