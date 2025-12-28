macro_rules! macro_901 {
    () => {
        bitflags ! { # [allow (missing_docs)] # [derive (Clone , Copy , Debug , Eq , PartialEq , PartialOrd , Ord , Hash)] pub struct StashApplyFlags : u32 { # [allow (missing_docs)] const DEFAULT = raw :: GIT_STASH_APPLY_DEFAULT as u32 ; # [doc = " Try to reinstate not only the working tree's changes,"] # [doc = " but also the index's changes."] const REINSTATE_INDEX = raw :: GIT_STASH_APPLY_REINSTATE_INDEX as u32 ; } }
    };
}

macro_901!();