// Generated macro for glob (function)
macro_rules! Depcrateglob {
() => {
// Module: crate
// Provides: {"glob"}
// Dependencies: {}
# [doc = " Construct a new `GlobWalker` with a glob pattern."] # [doc = ""] # [doc = " When iterated, the current directory will be recursively searched for paths"] # [doc = " matching `pattern`, unless the pattern specifies an absolute path."] pub fn glob < S : AsRef < str > > (pattern : S) -> Result < GlobWalker , GlobError > { glob_builder (pattern) . build () }
};
}
