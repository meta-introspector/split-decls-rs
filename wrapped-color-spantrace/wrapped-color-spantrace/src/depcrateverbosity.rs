// Generated macro for Verbosity (enum)
macro_rules! DepcrateVerbosity {
() => {
// Module: crate
// Provides: {"Verbosity"}
// Dependencies: {}
# [doc = " Defines how verbose the backtrace is supposed to be."] # [derive (Debug , Clone , Copy , PartialEq , Eq , PartialOrd , Ord)] enum Verbosity { # [doc = " Print a small message including the panic payload and the panic location."] Minimal , # [doc = " Everything in `Minimal` and additionally print a backtrace."] Medium , # [doc = " Everything in `Medium` plus source snippets for all backtrace locations."] Full , }
};
}
