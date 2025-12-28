macro_rules! WorktreeRoots {
    () => {
        # [doc = " A way to access roots for different kinds of resources that are possibly located and accessible in a worktree."] # [derive (Clone , Debug , Default)] pub struct WorktreeRoots { # [doc = " A place where the source of a rewrite, rename or copy, or generally the previous version of resources, are located."] pub old_root : Option < PathBuf > , # [doc = " A place where the destination of a rewrite, rename or copy, or generally the new version of resources, are located."] pub new_root : Option < PathBuf > , }
    };
}

WorktreeRoots!()