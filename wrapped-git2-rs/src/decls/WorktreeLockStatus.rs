macro_rules! deps {
    () => {
        Worktree!();
    };
}

macro_rules! WorktreeLockStatus {
    () => {
        deps!();
        # [doc = " Lock Status of a worktree"] # [derive (PartialEq , Debug)] pub enum WorktreeLockStatus { # [doc = " Worktree is Unlocked"] Unlocked , # [doc = " Worktree is locked with the optional message"] Locked (Option < String >) , }
    };
}

WorktreeLockStatus!();