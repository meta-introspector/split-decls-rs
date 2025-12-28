macro_rules! macro_18 {
    () => {
        bitflags ! { # [doc = " Flags for the `extended_flags` field of an IndexEntry."] # [derive (Clone , Copy , Debug , Eq , PartialEq , PartialOrd , Ord , Hash)] pub struct IndexEntryExtendedFlag : u16 { # [doc = " An \"intent to add\" entry from \"git add -N\""] const INTENT_TO_ADD = raw :: GIT_INDEX_ENTRY_INTENT_TO_ADD as u16 ; # [doc = " Skip the associated worktree file, for sparse checkouts"] const SKIP_WORKTREE = raw :: GIT_INDEX_ENTRY_SKIP_WORKTREE as u16 ; # [allow (missing_docs)] const UPTODATE = raw :: GIT_INDEX_ENTRY_UPTODATE as u16 ; } }
    };
}

macro_18!()