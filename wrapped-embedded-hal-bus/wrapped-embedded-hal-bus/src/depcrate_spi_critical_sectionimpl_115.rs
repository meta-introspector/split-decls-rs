// Generated macro for impl_115 (impl)
macro_rules! Depcrate_spi_critical_sectionimpl_115 {
() => {
// Module: crate::spi::critical_section
// Provides: {"impl_115"}
// Dependencies: {}
impl < 'a , BUS , CS , D > CriticalSectionDevice < 'a , BUS , CS , D > { # [doc = " Create a new [`CriticalSectionDevice`]."] # [doc = ""] # [doc = " This sets the `cs` pin high, and returns an error if that fails. It is recommended"] # [doc = " to set the pin high the moment it's configured as an output, to avoid glitches."] # [inline] pub fn new (bus : & 'a Mutex < RefCell < BUS > > , mut cs : CS , delay : D) -> Result < Self , CS :: Error > where CS : OutputPin , { cs . set_high () ? ; Ok (Self { bus , cs , delay }) } }
};
}
