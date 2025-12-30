// Generated macro for Revwalk (struct)
macro_rules! Depcrate_revwalkRevwalk {
() => {
// Module: crate::revwalk
// Provides: {"Revwalk"}
// Dependencies: {}
# [doc = " A revwalk allows traversal of the commit graph defined by including one or"] # [doc = " more leaves and excluding one or more roots."] pub struct Revwalk < 'repo > { raw : * mut raw :: git_revwalk , _marker : marker :: PhantomData < & 'repo Repository > , }
};
}
