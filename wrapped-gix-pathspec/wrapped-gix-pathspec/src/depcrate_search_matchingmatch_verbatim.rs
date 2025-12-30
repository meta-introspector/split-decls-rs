// Generated macro for match_verbatim (function)
macro_rules! Depcrate_search_matchingmatch_verbatim {
() => {
// Module: crate::search::matching
// Provides: {"match_verbatim"}
// Dependencies: {}
fn match_verbatim (mapping : & gix_glob :: search :: pattern :: Mapping < Spec > , relative_path : & BStr , is_dir : bool , case : Case , how : & mut MatchKind ,) -> bool { let pattern_len = mapping . value . pattern . path . len () ; let mut relative_path_ends_with_slash_at_pattern_len = false ; let (match_is_allowed , probably_how) = relative_path . get (pattern_len) . map_or_else (| | (relative_path . len () == pattern_len , Verbatim) , | b | { relative_path_ends_with_slash_at_pattern_len = * b == b'/' ; (relative_path_ends_with_slash_at_pattern_len , Prefix) } ,) ; * how = probably_how ; let pattern_requirement_is_met = ! mapping . pattern . mode . contains (gix_glob :: pattern :: Mode :: MUST_BE_DIR) || (relative_path_ends_with_slash_at_pattern_len || is_dir) ; if match_is_allowed && pattern_requirement_is_met { let dir_or_file = & relative_path [.. mapping . value . pattern . path . len ()] ; match case { Case :: Sensitive => mapping . value . pattern . path == dir_or_file , Case :: Fold => mapping . value . pattern . path . eq_ignore_ascii_case (dir_or_file) , } } else { false } }
};
}
