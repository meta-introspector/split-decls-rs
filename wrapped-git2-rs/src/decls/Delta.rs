macro_rules! Delta {
    () => {
        # [doc = " What type of change is described by a `DiffDelta`?"] # [derive (Copy , Clone , Debug , PartialEq , Eq)] pub enum Delta { # [doc = " No changes"] Unmodified , # [doc = " Entry does not exist in old version"] Added , # [doc = " Entry does not exist in new version"] Deleted , # [doc = " Entry content changed between old and new"] Modified , # [doc = " Entry was renamed between old and new"] Renamed , # [doc = " Entry was copied from another old entry"] Copied , # [doc = " Entry is ignored item in workdir"] Ignored , # [doc = " Entry is untracked item in workdir"] Untracked , # [doc = " Type of entry changed between old and new"] Typechange , # [doc = " Entry is unreadable"] Unreadable , # [doc = " Entry in the index is conflicted"] Conflicted , }
    };
}

Delta!()