macro_rules! DiffLineType {
    () => {
        # [doc = " Line origin constants."] # [derive (Copy , Clone , Debug , PartialEq)] pub enum DiffLineType { # [doc = " These values will be sent to `git_diff_line_cb` along with the line"] Context , # [doc = ""] Addition , # [doc = ""] Deletion , # [doc = " Both files have no LF at end"] ContextEOFNL , # [doc = " Old has no LF at end, new does"] AddEOFNL , # [doc = " Old has LF at end, new does not"] DeleteEOFNL , # [doc = " The following values will only be sent to a `git_diff_line_cb` when"] # [doc = " the content of a diff is being formatted through `git_diff_print`."] FileHeader , # [doc = ""] HunkHeader , # [doc = " For \"Binary files x and y differ\""] Binary , }
    };
}

DiffLineType!();