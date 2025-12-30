// Generated macro for impl_75 (impl)
macro_rules! Depcrate_spi_refcellimpl_75 {
() => {
// Module: crate::spi::refcell
// Provides: {"impl_75"}
// Dependencies: {}
impl < 'a , BUS , CS > RefCellDevice < 'a , BUS , CS , super :: NoDelay > { # [doc = " Create a new [`RefCellDevice`] without support for in-transaction delays."] # [doc = ""] # [doc = " This sets the `cs` pin high, and returns an error if that fails. It is recommended"] # [doc = " to set the pin high the moment it's configured as an output, to avoid glitches."] # [doc = ""] # [doc = " **Warning**: The returned instance *technically* doesn't comply with the `SpiDevice`"] # [doc = " contract, which mandates delay support. It is relatively rare for drivers to use"] # [doc = " in-transaction delays, so you might still want to use this method because it's more practical."] # [doc = ""] # [doc = " Note that a future version of the driver might start using delays, causing your"] # [doc = " code to panic. This wouldn't be considered a breaking change from the driver side, because"] # [doc = " drivers are allowed to assume `SpiDevice` implementations comply with the contract."] # [doc = " If you feel this risk outweighs the convenience of having `cargo` automatically upgrade"] # [doc = " the driver crate, you might want to pin the driver's version."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " The returned device will panic if you try to execute a transaction"] # [doc = " that contains any operations of type [`Operation::DelayNs`]."] # [inline] pub fn new_no_delay (bus : & 'a RefCell < BUS > , mut cs : CS) -> Result < Self , CS :: Error > where CS : OutputPin , { cs . set_high () ? ; Ok (Self { bus , cs , delay : super :: NoDelay , }) } }
};
}
