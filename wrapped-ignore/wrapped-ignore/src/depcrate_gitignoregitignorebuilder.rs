// Generated macro for GitignoreBuilder (struct)
macro_rules! Depcrate_gitignoreGitignoreBuilder {
() => {
// Module: crate::gitignore
// Provides: {"GitignoreBuilder"}
// Dependencies: {}
# [doc = " Builds a matcher for a single set of globs from a .gitignore file."] # [derive (Clone , Debug)] pub struct GitignoreBuilder { builder : GlobSetBuilder , root : PathBuf , globs : Vec < Glob > , case_insensitive : bool , allow_unclosed_class : bool , }
};
}
