// Generated macro for from_gitdir_file (function)
macro_rules! Depcrate_pathfrom_gitdir_file {
() => {
// Module: crate::path
// Provides: {"from_gitdir_file"}
// Dependencies: {}
# [doc = " Reads typical `gitdir: ` files from disk as used by worktrees and submodules."] pub fn from_gitdir_file (path : & std :: path :: Path) -> Result < PathBuf , from_gitdir_file :: Error > { let buf = read_regular_file_content_with_size_limit (path) ? ; let mut gitdir = crate :: parse :: gitdir (& buf) ? ; if let Some (parent) = path . parent () { gitdir = parent . join (gitdir) ; } Ok (gitdir) }
};
}
