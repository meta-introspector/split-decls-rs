// Generated macro for GEMError (enum)
macro_rules! Depcrate_drivers_net_gemGEMError {
() => {
// Module: crate::drivers::net::gem
// Provides: {"GEMError"}
// Dependencies: {}
# [derive (Error , Debug)] pub enum GEMError { # [error ("initialization failed")] InitFailed , # [error ("reset failed")] ResetFailed , # [error ("PHY not found")] NoPhyFound , # [error ("unknown GEM error")] Unknown , }
};
}
