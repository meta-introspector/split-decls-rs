macro_rules! deps {
    () => {
        MatchFinder!();
    };
}

macro_rules! assert_match_failure_reason {
    () => {
        deps!();
        fn assert_match_failure_reason (pattern : & str , code : & str , snippet : & str , expected_reason : & str) { let (db , position , selections) = single_file (code) ; let mut match_finder = MatchFinder :: in_context (& db , ide_db :: FilePosition { file_id : position . file_id . file_id (& db) , offset : position . offset } , selections . into_iter () . map (| selection | ide_db :: FileRange { file_id : selection . file_id . file_id (& db) , range : selection . range , }) . collect () ,) . unwrap () ; match_finder . add_search_pattern (pattern . parse () . unwrap ()) . unwrap () ; let mut reasons = Vec :: new () ; for d in match_finder . debug_where_text_equal (position . file_id , snippet) { if let Some (reason) = d . match_failure_reason () { reasons . push (reason . to_owned ()) ; } } assert_eq ! (reasons , vec ! [expected_reason]) ; }
    };
}

assert_match_failure_reason!();