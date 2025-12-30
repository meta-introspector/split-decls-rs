// Generated macro for Revspec (struct)
macro_rules! Depcrate_revspecRevspec {
() => {
// Module: crate::revspec
// Provides: {"Revspec"}
// Dependencies: {}
# [doc = " A revspec represents a range of revisions within a repository."] pub struct Revspec < 'repo > { from : Option < Object < 'repo > > , to : Option < Object < 'repo > > , mode : RevparseMode , }
};
}
