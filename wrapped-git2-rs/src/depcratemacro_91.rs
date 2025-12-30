// Generated macro for macro_91 (macro)
macro_rules! Depcratemacro_91 {
() => {
// Module: crate
// Provides: {"macro_91"}
// Dependencies: {}
bitflags ! { # [doc = " The results of `merge_analysis` indicating the merge opportunities."] # [derive (Clone , Copy , Debug , Eq , PartialEq , PartialOrd , Ord , Hash)] pub struct MergeAnalysis : u32 { # [doc = " No merge is possible."] const ANALYSIS_NONE = raw :: GIT_MERGE_ANALYSIS_NONE as u32 ; # [doc = " A \"normal\" merge; both HEAD and the given merge input have diverged"] # [doc = " from their common ancestor. The divergent commits must be merged."] const ANALYSIS_NORMAL = raw :: GIT_MERGE_ANALYSIS_NORMAL as u32 ; # [doc = " All given merge inputs are reachable from HEAD, meaning the"] # [doc = " repository is up-to-date and no merge needs to be performed."] const ANALYSIS_UP_TO_DATE = raw :: GIT_MERGE_ANALYSIS_UP_TO_DATE as u32 ; # [doc = " The given merge input is a fast-forward from HEAD and no merge"] # [doc = " needs to be performed.  Instead, the client can check out the"] # [doc = " given merge input."] const ANALYSIS_FASTFORWARD = raw :: GIT_MERGE_ANALYSIS_FASTFORWARD as u32 ; # [doc = " The HEAD of the current repository is \"unborn\" and does not point to"] # [doc = " a valid commit.  No merge can be performed, but the caller may wish"] # [doc = " to simply set HEAD to the target commit(s)."] const ANALYSIS_UNBORN = raw :: GIT_MERGE_ANALYSIS_UNBORN as u32 ; } }
};
}
