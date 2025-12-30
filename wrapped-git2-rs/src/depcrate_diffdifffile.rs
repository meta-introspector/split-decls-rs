// Generated macro for DiffFile (struct)
macro_rules! Depcrate_diffDiffFile {
() => {
// Module: crate::diff
// Provides: {"DiffFile"}
// Dependencies: {}
# [doc = " Description of one side of a delta."] # [doc = ""] # [doc = " Although this is called a \"file\" it could represent a file, a symbolic"] # [doc = " link, a submodule commit id, or even a tree (although that only happens if"] # [doc = " you are tracking type changes or ignored/untracked directories)."] pub struct DiffFile < 'a > { raw : * const raw :: git_diff_file , _marker : marker :: PhantomData < & 'a raw :: git_diff_file > , }
};
}
