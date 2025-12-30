// Generated macro for impl_74 (impl)
macro_rules! Depcrate_spi_refcellimpl_74 {
() => {
// Module: crate::spi::refcell
// Provides: {"impl_74"}
// Dependencies: {}
impl < 'a , BUS , CS , D > RefCellDevice < 'a , BUS , CS , D > { # [doc = " Create a new [`RefCellDevice`]."] # [doc = ""] # [doc = " This sets the `cs` pin high, and returns an error if that fails. It is recommended"] # [doc = " to set the pin high the moment it's configured as an output, to avoid glitches."] # [inline] pub fn new (bus : & 'a RefCell < BUS > , mut cs : CS , delay : D) -> Result < Self , CS :: Error > where CS : OutputPin , { cs . set_high () ? ; Ok (Self { bus , cs , delay }) } }
};
}
