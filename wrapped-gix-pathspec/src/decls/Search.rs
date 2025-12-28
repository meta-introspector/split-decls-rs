macro_rules! Search {
    () => {
        # [doc = " A lists of pathspec patterns, possibly from a file."] # [doc = ""] # [doc = " Pathspecs are generally relative to the root of the repository."] # [derive (Debug , Clone)] pub struct Search { # [doc = " Patterns and their associated data in the order they were loaded in or specified,"] # [doc = " the line number in its source file or its sequence number (_`(pattern, value, line_number)`_)."] # [doc = ""] # [doc = " During matching, this order is reversed."] patterns : Vec < gix_glob :: search :: pattern :: Mapping < search :: Spec > > , # [doc = " The path from which the patterns were read, or `None` if the patterns"] # [doc = " don't originate in a file on disk."] pub source : Option < PathBuf > , # [doc = " If `true`, this means all `patterns` are exclude patterns. This means that if there is no match"] # [doc = " (which would exclude an item), we would actually match it for lack of exclusion."] all_patterns_are_excluded : bool , # [doc = " The amount of bytes that are in common among all `patterns` and that aren't matched case-insensitively"] common_prefix_len : usize , }
    };
}

Search!()