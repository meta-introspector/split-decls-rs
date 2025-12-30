// Generated macro for BlameHunk (struct)
macro_rules! Depcrate_blameBlameHunk {
() => {
// Module: crate::blame
// Provides: {"BlameHunk"}
// Dependencies: {}
# [doc = " Structure that represents a blame hunk."] pub struct BlameHunk < 'blame > { raw : * mut raw :: git_blame_hunk , _marker : marker :: PhantomData < & 'blame raw :: git_blame > , }
};
}
