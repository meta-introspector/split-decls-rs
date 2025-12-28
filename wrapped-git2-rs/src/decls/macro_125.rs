macro_rules! macro_125 {
    () => {
        bitflags ! { # [doc = " Formatting options for diff stats"] # [derive (Clone , Copy , Debug , Eq , PartialEq , PartialOrd , Ord , Hash)] pub struct DiffStatsFormat : raw :: git_diff_stats_format_t { # [doc = " Don't generate any stats"] const NONE = raw :: GIT_DIFF_STATS_NONE ; # [doc = " Equivalent of `--stat` in git"] const FULL = raw :: GIT_DIFF_STATS_FULL ; # [doc = " Equivalent of `--shortstat` in git"] const SHORT = raw :: GIT_DIFF_STATS_SHORT ; # [doc = " Equivalent of `--numstat` in git"] const NUMBER = raw :: GIT_DIFF_STATS_NUMBER ; # [doc = " Extended header information such as creations, renames and mode"] # [doc = " changes, equivalent of `--summary` in git"] const INCLUDE_SUMMARY = raw :: GIT_DIFF_STATS_INCLUDE_SUMMARY ; } }
    };
}

macro_125!()