// Generated macro for git_clone_root_dir (function)
macro_rules! Depcrate_utilsgit_clone_root_dir {
() => {
// Module: crate::utils
// Provides: {"git_clone_root_dir"}
// Dependencies: {}
# [doc = " This function differs from `git_clone` in how it handles *where* the repository will be cloned."] # [doc = " In `git_clone`, it is cloned in the provided path. In this function, the path you provide is"] # [doc = " the parent folder. So if you pass \"a\" as folder and try to clone \"b.git\", it will be cloned into"] # [doc = " `a/b`."] pub fn git_clone_root_dir (to_clone : & str , dest_parent_dir : & Path , shallow_clone : bool ,) -> Result < CloneResult , String > { let repo_name = get_repo_name (to_clone) ; git_clone_inner (to_clone , & dest_parent_dir . join (& repo_name) , shallow_clone , repo_name) }
};
}
