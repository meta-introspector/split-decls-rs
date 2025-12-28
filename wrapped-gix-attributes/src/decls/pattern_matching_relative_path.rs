macro_rules! deps {
    () => {
        Outcome!();
        Assignments!();
        Value!();
        Attributes!();
    };
}

macro_rules! pattern_matching_relative_path {
    () => {
        deps!();
        # [doc = " Append all matches of patterns matching `relative_path` to `out`,"] # [doc = " providing a pre-computed `basename_pos` which is the starting position of the basename of `relative_path`."] # [doc = " `case` specifies whether cases should be folded during matching or not."] # [doc = " `is_dir` is true if `relative_path` is a directory."] # [doc = " Return `true` if at least one pattern matched."] # [allow (unused_variables)] fn pattern_matching_relative_path (list : & gix_glob :: search :: pattern :: List < Attributes > , relative_path : & BStr , basename_pos : Option < usize > , case : gix_glob :: pattern :: Case , is_dir : Option < bool > , out : & mut Outcome ,) -> bool { let (relative_path , basename_start_pos) = match list . strip_base_handle_recompute_basename_pos (relative_path , basename_pos , case) { Some (r) => r , None => return false , } ; let cur_len = out . remaining () ; 'outer : for pattern :: Mapping { pattern , value , sequence_number , } in list . patterns . iter () . rev () . filter (| pm | Attributes :: may_use_glob_pattern (& pm . pattern)) { let value : & Value = value ; let attrs = match value { Value :: MacroAssignments { .. } => { unreachable ! ("we can't match on macros as they have no pattern") } Value :: Assignments (attrs) => attrs , } ; if out . has_unspecified_attributes (attrs . iter () . map (| attr | attr . id)) && pattern . matches_repo_relative_path (relative_path , basename_start_pos , is_dir , case , gix_glob :: wildmatch :: Mode :: NO_MATCH_SLASH_LITERAL ,) { let all_filled = out . fill_attributes (attrs . iter () , pattern , list . source . as_ref () , * sequence_number) ; if all_filled { break 'outer ; } } } cur_len != out . remaining () }
    };
}

pattern_matching_relative_path!();