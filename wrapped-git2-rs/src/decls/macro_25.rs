macro_rules! macro_25 {
    () => {
        bitflags ! { # [doc = " Flags for the return value of `Repository::revparse`"] # [derive (Clone , Copy , Debug , Eq , PartialEq , PartialOrd , Ord , Hash)] pub struct RevparseMode : u32 { # [doc = " The spec targeted a single object"] const SINGLE = raw :: GIT_REVPARSE_SINGLE as u32 ; # [doc = " The spec targeted a range of commits"] const RANGE = raw :: GIT_REVPARSE_RANGE as u32 ; # [doc = " The spec used the `...` operator, which invokes special semantics."] const MERGE_BASE = raw :: GIT_REVPARSE_MERGE_BASE as u32 ; } }
    };
}

macro_25!();