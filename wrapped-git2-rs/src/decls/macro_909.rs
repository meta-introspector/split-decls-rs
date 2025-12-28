macro_rules! macro_909 {
    () => {
        bitflags ! { # [allow (missing_docs)] # [derive (Clone , Copy , Debug , Eq , PartialEq , PartialOrd , Ord , Hash)] pub struct DiffFlags : u32 { # [doc = " File(s) treated as binary data."] const BINARY = raw :: GIT_DIFF_FLAG_BINARY as u32 ; # [doc = " File(s) treated as text data."] const NOT_BINARY = raw :: GIT_DIFF_FLAG_NOT_BINARY as u32 ; # [doc = " `id` value is known correct."] const VALID_ID = raw :: GIT_DIFF_FLAG_VALID_ID as u32 ; # [doc = " File exists at this side of the delta."] const EXISTS = raw :: GIT_DIFF_FLAG_EXISTS as u32 ; } }
    };
}

macro_909!();