// Generated macro for DeviceError (enum)
macro_rules! Depcrate_spiDeviceError {
() => {
// Module: crate::spi
// Provides: {"DeviceError"}
// Dependencies: {}
# [doc = " Error type for [`ExclusiveDevice`] operations."] # [derive (Copy , Clone , Eq , PartialEq , Debug)] # [cfg_attr (feature = "defmt-03" , derive (defmt :: Format))] pub enum DeviceError < BUS , CS > { # [doc = " An inner SPI bus operation failed."] Spi (BUS) , # [doc = " Asserting or deasserting CS failed."] Cs (CS) , }
};
}
