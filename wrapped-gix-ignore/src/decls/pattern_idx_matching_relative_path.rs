macro_rules! deps {
    () => {
        Ignore!();
    };
}

macro_rules! pattern_idx_matching_relative_path {
    () => {
        deps!();
        # [doc = " Like [`pattern_matching_relative_path()`], but returns an index to the pattern"] # [doc = " that matched `relative_path`, instead of the match itself."] pub fn pattern_idx_matching_relative_path (list : & gix_glob :: search :: pattern :: List < Ignore > , relative_path : & BStr , basename_pos : Option < usize > , is_dir : Option < bool > , case : gix_glob :: pattern :: Case ,) -> Option < usize > { let (relative_path , basename_start_pos) = list . strip_base_handle_recompute_basename_pos (relative_path , basename_pos , case) ? ; list . patterns . iter () . enumerate () . rev () . find_map (| (idx , pm) | { pm . pattern . matches_repo_relative_path (relative_path , basename_start_pos , is_dir , case , gix_glob :: wildmatch :: Mode :: NO_MATCH_SLASH_LITERAL ,) . then_some (idx) }) }
    };
}

pattern_idx_matching_relative_path!();