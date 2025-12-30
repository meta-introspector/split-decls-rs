// Generated macro for Gitignore (struct)
macro_rules! Depcrate_gitignoreGitignore {
() => {
// Module: crate::gitignore
// Provides: {"Gitignore"}
// Dependencies: {}
# [doc = " Gitignore is a matcher for the globs in one or more gitignore files"] # [doc = " in the same directory."] # [derive (Clone , Debug)] pub struct Gitignore { set : GlobSet , root : PathBuf , globs : Vec < Glob > , num_ignores : u64 , num_whitelists : u64 , matches : Option < Arc < Pool < Vec < usize > > > > , }
};
}
