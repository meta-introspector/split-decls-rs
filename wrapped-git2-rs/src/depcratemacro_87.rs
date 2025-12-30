// Generated macro for macro_87 (macro)
macro_rules! Depcratemacro_87 {
() => {
// Module: crate
// Provides: {"macro_87"}
// Dependencies: {}
bitflags ! { # [doc = " Flags for `Repository::open_ext`"] # [derive (Clone , Copy , Debug , Eq , PartialEq , PartialOrd , Ord , Hash)] pub struct RepositoryOpenFlags : u32 { # [doc = " Only open the specified path; don't walk upward searching."] const NO_SEARCH = raw :: GIT_REPOSITORY_OPEN_NO_SEARCH as u32 ; # [doc = " Search across filesystem boundaries."] const CROSS_FS = raw :: GIT_REPOSITORY_OPEN_CROSS_FS as u32 ; # [doc = " Force opening as bare repository, and defer loading its config."] const BARE = raw :: GIT_REPOSITORY_OPEN_BARE as u32 ; # [doc = " Don't try appending `/.git` to the specified repository path."] const NO_DOTGIT = raw :: GIT_REPOSITORY_OPEN_NO_DOTGIT as u32 ; # [doc = " Respect environment variables like `$GIT_DIR`."] const FROM_ENV = raw :: GIT_REPOSITORY_OPEN_FROM_ENV as u32 ; } }
};
}
