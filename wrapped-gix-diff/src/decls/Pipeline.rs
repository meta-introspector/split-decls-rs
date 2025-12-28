macro_rules! deps {
    () => {
        Outcome!();
        WorktreeRoots!();
        Driver!();
        Options!();
    };
}

macro_rules! Pipeline {
    () => {
        deps!();
        # [doc = " A conversion pipeline to take an object or path from what's stored in `git` to what can be diffed, while"] # [doc = " following the guidance of git-attributes at the respective path to learn if diffing should happen or if"] # [doc = " the content is considered binary."] # [doc = ""] # [doc = " There are two different conversion flows, where the target of the flow is a buffer with diffable content:"] # [doc = ""] # [doc = " * `worktree on disk` -> `text conversion`"] # [doc = " * `object` -> `worktree-filters` -> `text conversion`"] # [derive (Clone)] pub struct Pipeline { # [doc = " A way to read data directly from the worktree."] pub roots : pipeline :: WorktreeRoots , # [doc = " A pipeline to convert objects from what's stored in `git` to its worktree version."] pub worktree_filter : gix_filter :: Pipeline , # [doc = " Options affecting the way we read files."] pub options : pipeline :: Options , # [doc = " Drivers to help customize the conversion behaviour depending on the location of items."] drivers : Vec < Driver > , # [doc = " Pre-configured attributes to obtain additional diff-related information."] attrs : gix_filter :: attributes :: search :: Outcome , # [doc = " A buffer to manipulate paths"] path : PathBuf , }
    };
}

Pipeline!()