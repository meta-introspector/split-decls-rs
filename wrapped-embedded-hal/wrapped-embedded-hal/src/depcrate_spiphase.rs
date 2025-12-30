// Generated macro for Phase (enum)
macro_rules! Depcrate_spiPhase {
() => {
// Module: crate::spi
// Provides: {"Phase"}
// Dependencies: {}
# [doc = " Clock phase."] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [cfg_attr (feature = "defmt-03" , derive (defmt :: Format))] pub enum Phase { # [doc = " Data in \"captured\" on the first clock transition."] CaptureOnFirstTransition , # [doc = " Data in \"captured\" on the second clock transition."] CaptureOnSecondTransition , }
};
}
