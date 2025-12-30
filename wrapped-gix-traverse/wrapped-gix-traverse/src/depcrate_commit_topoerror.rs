// Generated macro for Error (enum)
macro_rules! Depcrate_commit_topoError {
() => {
// Module: crate::commit::topo
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The errors that can occur during creation and iteration."] # [derive (thiserror :: Error , Debug)] # [allow (missing_docs)] pub enum Error { # [error ("Indegree information is missing")] MissingIndegreeUnexpected , # [error ("Internal state (bitflags) not found")] MissingStateUnexpected , # [error (transparent)] ObjectDecode (# [from] gix_object :: decode :: Error) , # [error (transparent)] Find (# [from] gix_object :: find :: existing_iter :: Error) , }
};
}
