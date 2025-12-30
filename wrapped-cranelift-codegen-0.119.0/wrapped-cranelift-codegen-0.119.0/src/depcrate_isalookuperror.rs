// Generated macro for LookupError (enum)
macro_rules! Depcrate_isaLookupError {
() => {
// Module: crate::isa
// Provides: {"LookupError"}
// Dependencies: {}
# [doc = " Describes reason for target lookup failure"] # [derive (PartialEq , Eq , Copy , Clone , Debug)] pub enum LookupError { # [doc = " Support for this target was disabled in the current build."] SupportDisabled , # [doc = " Support for this target has not yet been implemented."] Unsupported , }
};
}
