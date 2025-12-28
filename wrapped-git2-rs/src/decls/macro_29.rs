macro_rules! macro_29 {
    () => {
        bitflags ! { # [doc = " The user's stated preference for merges."] # [derive (Clone , Copy , Debug , Eq , PartialEq , PartialOrd , Ord , Hash)] pub struct MergePreference : u32 { # [doc = " No configuration was found that suggests a preferred behavior for"] # [doc = " merge."] const NONE = raw :: GIT_MERGE_PREFERENCE_NONE as u32 ; # [doc = " There is a `merge.ff=false` configuration setting, suggesting that"] # [doc = " the user does not want to allow a fast-forward merge."] const NO_FAST_FORWARD = raw :: GIT_MERGE_PREFERENCE_NO_FASTFORWARD as u32 ; # [doc = " There is a `merge.ff=only` configuration setting, suggesting that"] # [doc = " the user only wants fast-forward merges."] const FASTFORWARD_ONLY = raw :: GIT_MERGE_PREFERENCE_FASTFORWARD_ONLY as u32 ; } }
    };
}

macro_29!();