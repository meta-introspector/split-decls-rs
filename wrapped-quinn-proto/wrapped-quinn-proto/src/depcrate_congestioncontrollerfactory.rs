// Generated macro for ControllerFactory (trait)
macro_rules! Depcrate_congestionControllerFactory {
() => {
// Module: crate::congestion
// Provides: {"ControllerFactory"}
// Dependencies: {}
# [doc = " Constructs controllers on demand"] pub trait ControllerFactory { # [doc = " Construct a fresh `Controller`"] fn build (self : Arc < Self > , now : Instant , current_mtu : u16) -> Box < dyn Controller > ; }
};
}
