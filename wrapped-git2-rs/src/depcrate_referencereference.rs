// Generated macro for Reference (struct)
macro_rules! Depcrate_referenceReference {
() => {
// Module: crate::reference
// Provides: {"Reference"}
// Dependencies: {}
# [doc = " A structure to represent a git [reference][1]."] # [doc = ""] # [doc = " [1]: http://git-scm.com/book/en/Git-Internals-Git-References"] pub struct Reference < 'repo > { raw : * mut raw :: git_reference , _marker : marker :: PhantomData < Refdb < 'repo > > , }
};
}
