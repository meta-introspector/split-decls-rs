// Generated macro for impl_60 (impl)
macro_rules! Depcrate_spi_exclusiveimpl_60 {
() => {
// Module: crate::spi::exclusive
// Provides: {"impl_60"}
// Dependencies: {}
impl < BUS , CS , D > ExclusiveDevice < BUS , CS , D > { # [doc = " Create a new [`ExclusiveDevice`]."] # [doc = ""] # [doc = " This sets the `cs` pin high, and returns an error if that fails. It is recommended"] # [doc = " to set the pin high the moment it's configured as an output, to avoid glitches."] # [inline] pub fn new (bus : BUS , mut cs : CS , delay : D) -> Result < Self , CS :: Error > where CS : OutputPin , { cs . set_high () ? ; Ok (Self { bus , cs , delay }) } # [doc = " Returns a reference to the underlying bus object."] # [inline] pub fn bus (& self) -> & BUS { & self . bus } # [doc = " Returns a mutable reference to the underlying bus object."] # [inline] pub fn bus_mut (& mut self) -> & mut BUS { & mut self . bus } }
};
}
