// Generated macro for ExclusiveDevice (struct)
macro_rules! Depcrate_spi_exclusiveExclusiveDevice {
() => {
// Module: crate::spi::exclusive
// Provides: {"ExclusiveDevice"}
// Dependencies: {}
# [doc = " [`SpiDevice`] implementation with exclusive access to the bus (not shared)."] # [doc = ""] # [doc = " This is the most straightforward way of obtaining an [`SpiDevice`] from an [`SpiBus`],"] # [doc = " ideal for when no sharing is required (only one SPI device is present on the bus)."] pub struct ExclusiveDevice < BUS , CS , D > { bus : BUS , cs : CS , delay : D , }
};
}
