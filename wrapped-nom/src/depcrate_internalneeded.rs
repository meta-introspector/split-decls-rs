// Generated macro for Needed (enum)
macro_rules! Depcrate_internalNeeded {
() => {
// Module: crate::internal
// Provides: {"Needed"}
// Dependencies: {}
# [doc = " Contains information on needed data if a parser returned `Incomplete`"] # [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum Needed { # [doc = " Needs more data, but we do not know how much"] Unknown , # [doc = " Contains the required data size in bytes"] Size (NonZeroUsize) , }
};
}
