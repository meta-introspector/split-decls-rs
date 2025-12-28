macro_rules! macro_20 {
    () => {
        bitflags ! { # [doc = " Flags for APIs that add files matching pathspec"] # [derive (Clone , Copy , Debug , Eq , PartialEq , PartialOrd , Ord , Hash)] pub struct IndexAddOption : u32 { # [doc = " Adds files that are not ignored to the index"] const DEFAULT = raw :: GIT_INDEX_ADD_DEFAULT as u32 ; # [doc = " Allows adding otherwise ignored files to the index"] const FORCE = raw :: GIT_INDEX_ADD_FORCE as u32 ; # [allow (missing_docs)] const DISABLE_PATHSPEC_MATCH = raw :: GIT_INDEX_ADD_DISABLE_PATHSPEC_MATCH as u32 ; # [allow (missing_docs)] const CHECK_PATHSPEC = raw :: GIT_INDEX_ADD_CHECK_PATHSPEC as u32 ; } }
    };
}

macro_20!();