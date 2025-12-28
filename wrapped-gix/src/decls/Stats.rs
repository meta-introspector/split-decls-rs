macro_rules! deps {
    () => {
        Default!();
        Clone!();
    };
}

macro_rules! Stats {
    () => {
        deps!();
        # [doc = " Provide aggregated information of a diff between two trees."] # [derive (Default , Debug , Copy , Clone , Ord , PartialOrd , Eq , PartialEq , Hash)] # [doc (alias = "DiffStats" , alias = "git2")] pub struct Stats { # [doc = " The total amount of lines added in the between blobs of the two trees."] # [doc (alias = "insertions" , alias = "git2")] pub lines_added : u64 , # [doc = " The total amount of lines removed in the between blobs of the two trees."] # [doc (alias = "deletions" , alias = "git2")] pub lines_removed : u64 , # [doc = " The number of files that contributed to these statistics as they were added, removed or modified."] pub files_changed : u64 , }
    };
}

Stats!();