// Generated macro for Mode (enum)
macro_rules! Depcrate_typesMode {
() => {
// Module: crate::types
// Provides: {"Mode"}
// Dependencies: {}
# [doc = " The way to interpret a refspec."] # [derive (PartialOrd , Ord , PartialEq , Eq , Copy , Clone , Hash , Debug)] pub (crate) enum Mode { # [doc = " Apply standard rules for refspecs which are including refs with specific rules related to allowing fast forwards of destinations."] Normal , # [doc = " Even though according to normal rules a non-fastforward would be denied, override this and reset a ref forcefully in the destination."] Force , # [doc = " Instead of considering matching refs included, we consider them excluded. This applies only to the source side of a refspec."] Negative , }
};
}
