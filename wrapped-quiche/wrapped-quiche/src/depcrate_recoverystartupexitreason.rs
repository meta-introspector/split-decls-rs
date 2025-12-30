// Generated macro for StartupExitReason (enum)
macro_rules! Depcrate_recoveryStartupExitReason {
() => {
// Module: crate::recovery
// Provides: {"StartupExitReason"}
// Dependencies: {}
# [doc = " The reason a CCA exited the startup phase."] # [derive (Debug , Clone , Copy , PartialEq)] pub enum StartupExitReason { # [doc = " Exit startup due to excessive loss"] Loss , # [doc = " Exit startup due to bandwidth plateau."] BandwidthPlateau , # [doc = " Exit startup due to persistent queue."] PersistentQueue , }
};
}
