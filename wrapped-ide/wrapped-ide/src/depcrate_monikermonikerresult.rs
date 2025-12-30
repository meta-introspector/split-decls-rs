// Generated macro for MonikerResult (enum)
macro_rules! Depcrate_monikerMonikerResult {
() => {
// Module: crate::moniker
// Provides: {"MonikerResult"}
// Dependencies: {}
# [derive (Debug , Clone , PartialEq , Eq , Hash)] pub enum MonikerResult { # [doc = " Uniquely identifies a definition."] Moniker (Moniker) , # [doc = " Specifies that the definition is a local, and so does not have a unique identifier. Provides"] # [doc = " a unique identifier for the container."] Local { enclosing_moniker : Option < Moniker > } , }
};
}
