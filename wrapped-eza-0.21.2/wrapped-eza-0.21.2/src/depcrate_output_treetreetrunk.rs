// Generated macro for TreeTrunk (struct)
macro_rules! Depcrate_output_treeTreeTrunk {
() => {
// Module: crate::output::tree
// Provides: {"TreeTrunk"}
// Dependencies: {}
# [doc = " A **tree trunk** builds up arrays of tree parts over multiple depths."] # [derive (Debug , Default)] pub struct TreeTrunk { # [doc = " A stack tracks which tree characters should be printed. It’s"] # [doc = " necessary to maintain information about the previously-printed"] # [doc = " lines, as the output will change based on any previous entries."] stack : Vec < TreePart > , # [doc = " A tuple for the last ‘depth’ and ‘last’ parameters that are passed in."] last_params : Option < TreeParams > , }
};
}
