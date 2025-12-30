// Generated macro for Mode (struct)
macro_rules! Depcrate_spiMode {
() => {
// Module: crate::spi
// Provides: {"Mode"}
// Dependencies: {}
# [doc = " SPI mode."] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [cfg_attr (feature = "defmt-03" , derive (defmt :: Format))] pub struct Mode { # [doc = " Clock polarity."] pub polarity : Polarity , # [doc = " Clock phase."] pub phase : Phase , }
};
}
