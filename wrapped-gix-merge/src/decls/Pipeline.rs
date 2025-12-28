macro_rules! deps {
    () => {
        WorktreeRoots!();
        Options!();
    };
}

macro_rules! Pipeline {
    () => {
        deps!();
        # [doc = " A conversion pipeline to take an object or path from what's stored in Git to what can be merged, while"] # [doc = " following the guidance of git-attributes at the respective path to learn how the merge should be performed."] # [doc = ""] # [doc = " Depending on the source, different conversions are performed:"] # [doc = ""] # [doc = " * `worktree on disk` -> `object for storage in git`"] # [doc = " * `object` -> `possibly renormalized object`"] # [doc = "     - Renormalization means that the `object` is converted to what would be checked out into the work-tree,"] # [doc = "       just to turn it back into an object."] # [derive (Clone)] pub struct Pipeline { # [doc = " A way to read data directly from the worktree."] pub roots : pipeline :: WorktreeRoots , # [doc = " A pipeline to convert objects from the worktree to Git, and also from Git to the worktree, and back to Git."] pub filter : gix_filter :: Pipeline , # [doc = " Options affecting the way we read files."] pub options : pipeline :: Options , # [doc = " A buffer to produce disk-accessible paths from worktree roots."] path : PathBuf , }
    };
}

Pipeline!()