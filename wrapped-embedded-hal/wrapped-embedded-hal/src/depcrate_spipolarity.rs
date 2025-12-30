// Generated macro for Polarity (enum)
macro_rules! Depcrate_spiPolarity {
() => {
// Module: crate::spi
// Provides: {"Polarity"}
// Dependencies: {}
# [doc = " Clock polarity."] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [cfg_attr (feature = "defmt-03" , derive (defmt :: Format))] pub enum Polarity { # [doc = " Clock signal low when idle."] IdleLow , # [doc = " Clock signal high when idle."] IdleHigh , }
};
}
