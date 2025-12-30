// Generated macro for pattern_matching_relative_path (function)
macro_rules! Depcrate_searchpattern_matching_relative_path {
() => {
// Module: crate::search
// Provides: {"pattern_matching_relative_path"}
// Dependencies: {}
# [doc = " Return a match if a pattern matches `relative_path`, providing a pre-computed `basename_pos` which is the"] # [doc = " starting position of the basename of `relative_path`. `is_dir` is true if `relative_path` is a directory."] # [doc = " `case` specifies whether cases should be folded during matching or not."] pub fn pattern_matching_relative_path < 'a > (list : & 'a gix_glob :: search :: pattern :: List < Ignore > , relative_path : & BStr , basename_pos : Option < usize > , is_dir : Option < bool > , case : gix_glob :: pattern :: Case ,) -> Option < Match < 'a > > { let (relative_path , basename_start_pos) = list . strip_base_handle_recompute_basename_pos (relative_path , basename_pos , case) ? ; list . patterns . iter () . rev () . find_map (| pattern :: Mapping { pattern , value : kind , sequence_number , } | { pattern . matches_repo_relative_path (relative_path , basename_start_pos , is_dir , case , gix_glob :: wildmatch :: Mode :: NO_MATCH_SLASH_LITERAL ,) . then_some (Match { pattern , kind : * kind , source : list . source . as_deref () , sequence_number : * sequence_number , }) } ,) }
};
}
