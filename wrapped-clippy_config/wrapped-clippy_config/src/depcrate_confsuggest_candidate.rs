// Generated macro for suggest_candidate (function)
macro_rules! Depcrate_confsuggest_candidate {
() => {
// Module: crate::conf
// Provides: {"suggest_candidate"}
// Dependencies: {}
# [doc = " Given a user-provided value that couldn't be matched to a known option, finds the most likely"] # [doc = " candidate among candidates that the user might have meant."] fn suggest_candidate < 'a , I > (value : & str , candidates : I) -> Option < & 'a str > where I : IntoIterator < Item = & 'a str > , { candidates . into_iter () . filter_map (| expected | { let dist = edit_distance (value , expected , 4) ? ; Some ((dist , expected)) }) . min_by_key (| & (dist , _) | dist) . map (| (_ , suggestion) | suggestion) }
};
}
