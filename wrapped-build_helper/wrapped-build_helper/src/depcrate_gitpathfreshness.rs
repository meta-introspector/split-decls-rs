// Generated macro for PathFreshness (enum)
macro_rules! Depcrate_gitPathFreshness {
() => {
// Module: crate::git
// Provides: {"PathFreshness"}
// Dependencies: {}
# [doc = " Represents the result of checking whether a set of paths"] # [doc = " have been modified locally or not."] # [derive (PartialEq , Debug , Clone)] pub enum PathFreshness { # [doc = " Artifacts should be downloaded from this upstream commit,"] # [doc = " there are no local modifications."] LastModifiedUpstream { upstream : String } , # [doc = " There are local modifications to a certain set of paths."] # [doc = " \"Local\" essentially means \"not-upstream\" here."] # [doc = " `upstream` is the latest upstream merge commit that made modifications to the"] # [doc = " set of paths."] HasLocalModifications { upstream : String } , # [doc = " No upstream commit was found."] # [doc = " This should not happen in most reasonable circumstances, but one never knows."] MissingUpstream , }
};
}
