macro_rules! HunkHeader {
    () => {
        # [doc = " Holds information about a unified diff hunk, specifically with respect to line numbers."] # [derive (Default , Debug , Copy , Clone , PartialEq , Eq , Hash , Ord , PartialOrd)] pub struct HunkHeader { # [doc = " The 1-based start position in the 'before' lines."] pub before_hunk_start : u32 , # [doc = " The size of the 'before' hunk in lines."] pub before_hunk_len : u32 , # [doc = " The 1-based start position in the 'after' lines."] pub after_hunk_start : u32 , # [doc = " The size of the 'after' hunk in lines."] pub after_hunk_len : u32 , }
    };
}

HunkHeader!();