// Generated macro for ResourceKind (enum)
macro_rules! Depcrate_blobResourceKind {
() => {
// Module: crate::blob
// Provides: {"ResourceKind"}
// Dependencies: {}
# [doc = " A way to classify the side of a resource for merging."] # [derive (Copy , Clone , Debug , Ord , PartialOrd , Eq , PartialEq , Hash)] pub enum ResourceKind { # [doc = " Our side of the state."] CurrentOrOurs , # [doc = " Their side of the state."] OtherOrTheirs , # [doc = " The state of the common base of both ours and theirs."] CommonAncestorOrBase , }
};
}
