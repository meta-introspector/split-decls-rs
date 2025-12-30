// Generated macro for SpecIndex (enum)
macro_rules! Depcrate_fetch_refmapSpecIndex {
() => {
// Module: crate::fetch::refmap
// Provides: {"SpecIndex"}
// Dependencies: {}
# [doc = " An index into various lists of refspecs that have been used in a [Mapping] of remote references to local ones."] # [derive (Debug , Copy , Clone , Eq , PartialEq , Hash , Ord , PartialOrd)] pub enum SpecIndex { # [doc = " An index into the _refspecs of the remote_ that triggered a fetch operation."] # [doc = " These refspecs are explicit and visible to the user."] ExplicitInRemote (usize) , # [doc = " An index into the list of [extra refspecs](crate::fetch::RefMap::extra_refspecs) that are implicit"] # [doc = " to a particular fetch operation."] Implicit (usize) , }
};
}
