macro_rules! UnblamedHunk {
    () => {
        # [doc = " Tracks the hunks in the *Blamed File* that are not yet associated with the commit that introduced them."] # [derive (Debug , PartialEq)] pub struct UnblamedHunk { # [doc = " The range in the file that is being blamed that this hunk represents."] pub range_in_blamed_file : Range < u32 > , # [doc = " Maps a commit to the range in a source file (i.e. *Blamed File* at a revision) that is"] # [doc = " equal to `range_in_blamed_file`. Since `suspects` rarely contains more than 1 item, it can"] # [doc = " efficiently be stored as a `SmallVec`."] pub suspects : SmallVec < (ObjectId , Range < u32 >) , 1 > , # [doc = " The *Source File*'s name, in case it differs from *Blamed File*'s name."] pub source_file_name : Option < BString > , }
    };
}

UnblamedHunk!()