// Generated macro for Commit (struct)
macro_rules! Depcrate_commitCommit {
() => {
// Module: crate::commit
// Provides: {"Commit"}
// Dependencies: {}
# [doc = " A structure to represent a git [commit][1]"] # [doc = ""] # [doc = " [1]: http://git-scm.com/book/en/Git-Internals-Git-Objects"] pub struct Commit < 'repo > { raw : * mut raw :: git_commit , _marker : marker :: PhantomData < Object < 'repo > > , }
};
}
