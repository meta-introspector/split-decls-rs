macro_rules! deps {
    () => {
        Kind!();
        PathspecMatch!();
        Property!();
    };
}

macro_rules! Status {
    () => {
        deps!();
        # [doc = " The kind of entry as obtained from a directory."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash , Ord , PartialOrd)] pub enum Status { # [doc = " The entry was removed from the walk due to its other properties, like [Property] or [PathspecMatch]"] # [doc = ""] # [doc = " Note that entries flagged as `DotGit` directory will always be considered `Pruned`, but if they are"] # [doc = " also ignored, in delete mode, they will be considered `Ignored` instead. This way, it's easier to remove them"] # [doc = " while they will not be available for any interactions in read-only mode."] Pruned , # [doc = " The entry is tracked in Git."] Tracked , # [doc = " The entry is ignored as per `.gitignore` files and their rules."] # [doc = ""] # [doc = " If this is a directory, then its entire contents is ignored. Otherwise, possibly due to configuration, individual ignored files are listed."] Ignored (gix_ignore :: Kind) , # [doc = " The entry is not tracked by git yet, it was not found in the [index](gix_index::State)."] # [doc = ""] # [doc = " If it's a directory, the entire directory contents is untracked."] Untracked , }
    };
}

Status!();