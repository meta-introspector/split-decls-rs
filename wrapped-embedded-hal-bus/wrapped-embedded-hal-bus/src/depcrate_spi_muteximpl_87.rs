// Generated macro for impl_87 (impl)
macro_rules! Depcrate_spi_muteximpl_87 {
() => {
// Module: crate::spi::mutex
// Provides: {"impl_87"}
// Dependencies: {}
impl < 'a , BUS , CS , D > MutexDevice < 'a , BUS , CS , D > { # [doc = " Create a new [`MutexDevice`]."] # [doc = ""] # [doc = " This sets the `cs` pin high, and returns an error if that fails. It is recommended"] # [doc = " to set the pin high the moment it's configured as an output, to avoid glitches."] # [inline] pub fn new (bus : & 'a Mutex < BUS > , mut cs : CS , delay : D) -> Result < Self , CS :: Error > where CS : OutputPin , { cs . set_high () ? ; Ok (Self { bus , cs , delay }) } }
};
}
