// Generated macro for PathspecMatchList (struct)
macro_rules! Depcrate_pathspecPathspecMatchList {
() => {
// Module: crate::pathspec
// Provides: {"PathspecMatchList"}
// Dependencies: {}
# [doc = " List of filenames matching a pathspec."] pub struct PathspecMatchList < 'ps > { raw : * mut raw :: git_pathspec_match_list , _marker : marker :: PhantomData < & 'ps Pathspec > , }
};
}
