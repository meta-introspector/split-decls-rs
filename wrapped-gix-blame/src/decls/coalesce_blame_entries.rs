macro_rules! deps {
    () => {
        BlameEntry!();
    };
}

macro_rules! coalesce_blame_entries {
    () => {
        deps!();
        # [doc = " This function merges adjacent blame entries. It merges entries that are adjacent both in the"] # [doc = " blamed file and in the source file that introduced them. This follows `git`’s"] # [doc = " behaviour. `libgit2`, as of 2024-09-19, only checks whether two entries are adjacent in the"] # [doc = " blamed file which can result in different blames in certain edge cases. See [the commit][1]"] # [doc = " that introduced the extra check into `git` for context. See [this commit][2] for a way to test"] # [doc = " for this behaviour in `git`."] # [doc = ""] # [doc = " [1]: https://github.com/git/git/commit/c2ebaa27d63bfb7c50cbbdaba90aee4efdd45d0a"] # [doc = " [2]: https://github.com/git/git/commit/6dbf0c7bebd1c71c44d786ebac0f2b3f226a0131"] fn coalesce_blame_entries (lines_blamed : Vec < BlameEntry >) -> Vec < BlameEntry > { let len = lines_blamed . len () ; lines_blamed . into_iter () . fold (Vec :: with_capacity (len) , | mut acc , entry | { let previous_entry = acc . last () ; if let Some (previous_entry) = previous_entry { let previous_blamed_range = previous_entry . range_in_blamed_file () ; let current_blamed_range = entry . range_in_blamed_file () ; let previous_source_range = previous_entry . range_in_source_file () ; let current_source_range = entry . range_in_source_file () ; if previous_entry . commit_id == entry . commit_id && previous_blamed_range . end == current_blamed_range . start && previous_source_range . end == current_source_range . start { let coalesced_entry = BlameEntry { start_in_blamed_file : previous_blamed_range . start as u32 , start_in_source_file : previous_source_range . start as u32 , len : NonZeroU32 :: new ((current_source_range . end - previous_source_range . start) as u32) . expect ("BUG: hunks are never zero-sized") , commit_id : previous_entry . commit_id , source_file_name : previous_entry . source_file_name . clone () , } ; acc . pop () ; acc . push (coalesced_entry) ; } else { acc . push (entry) ; } acc } else { acc . push (entry) ; acc } }) }
    };
}

coalesce_blame_entries!();