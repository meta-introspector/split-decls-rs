// Generated macro for Fallback (enum)
macro_rules! Depcrate_displaynames_optionsFallback {
() => {
// Module: crate::displaynames::options
// Provides: {"Fallback"}
// Dependencies: {}
# [doc = " An enum for fallback return when the system does not have the"] # [doc = " requested display name."] # [allow (missing_docs)] # [non_exhaustive] # [derive (Debug , Default , Eq , PartialEq , Clone , Copy)] pub enum Fallback { # [doc = " Fall back to the BCP-47 code when display name cannot be found"] # [default] Code , # [doc = " Do not fall back, return an error when the display name cannot be found"] None , }
};
}
