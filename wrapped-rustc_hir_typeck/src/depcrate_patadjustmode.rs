// Generated macro for AdjustMode (enum)
macro_rules! Depcrate_patAdjustMode {
() => {
// Module: crate::pat
// Provides: {"AdjustMode"}
// Dependencies: {}
# [doc = " Mode for adjusting the expected type and binding mode."] # [derive (Clone , Copy , Debug , PartialEq , Eq)] enum AdjustMode { # [doc = " Peel off all immediate reference types. If the `deref_patterns` feature is enabled, this"] # [doc = " also peels smart pointer ADTs."] Peel { kind : PeelKind } , # [doc = " Pass on the input binding mode and expected type."] Pass , }
};
}
