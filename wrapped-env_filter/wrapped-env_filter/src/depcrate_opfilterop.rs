// Generated macro for FilterOp (struct)
macro_rules! Depcrate_opFilterOp {
() => {
// Module: crate::op
// Provides: {"FilterOp"}
// Dependencies: {}
# [derive (Debug , Clone)] pub (crate) struct FilterOp { # [cfg (feature = "regex")] inner : regex :: Regex , # [cfg (not (feature = "regex"))] inner : String , }
};
}
