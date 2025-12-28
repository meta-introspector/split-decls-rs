macro_rules! WorktreeRoots {
    () => {
        # [doc = " A way to access roots for different kinds of resources that are possibly located and accessible in a worktree."] # [derive (Clone , Debug , Default)] pub struct WorktreeRoots { # [doc = " The worktree root where the current (or our) version of the resource is present."] pub current_root : Option < PathBuf > , # [doc = " The worktree root where the other (or their) version of the resource is present."] pub other_root : Option < PathBuf > , # [doc = " The worktree root where containing the resource of the common ancestor of our and their version."] pub common_ancestor_root : Option < PathBuf > , }
    };
}

WorktreeRoots!();