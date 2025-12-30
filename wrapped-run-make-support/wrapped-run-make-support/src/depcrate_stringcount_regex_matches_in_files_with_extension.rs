// Generated macro for count_regex_matches_in_files_with_extension (function)
macro_rules! Depcrate_stringcount_regex_matches_in_files_with_extension {
() => {
// Module: crate::string
// Provides: {"count_regex_matches_in_files_with_extension"}
// Dependencies: {}
# [doc = " Gathers all files in the current working directory that have the extension `ext`, and counts"] # [doc = " the number of lines within that contain a match with the regex pattern `re`."] pub fn count_regex_matches_in_files_with_extension (re : & regex :: Regex , ext : & str) -> usize { let fetched_files = shallow_find_files (cwd () , | path | has_extension (path , ext)) ; let mut count = 0 ; for file in fetched_files { let content = fs :: read_to_string (file) ; count += content . lines () . filter (| line | re . is_match (& line)) . count () ; } count }
};
}
