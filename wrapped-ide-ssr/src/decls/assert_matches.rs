macro_rules! deps {
    () => {
        MatchFinder!();
    };
}

macro_rules! assert_matches {
    () => {
        deps!();
        fn assert_matches (pattern : & str , code : & str , expected : & [& str]) { let (db , position , selections) = single_file (code) ; hir :: attach_db (& db , | | { let mut match_finder = MatchFinder :: in_context (& db , ide_db :: FilePosition { file_id : position . file_id . file_id (& db) , offset : position . offset , } , selections . into_iter () . map (| selection | ide_db :: FileRange { file_id : selection . file_id . file_id (& db) , range : selection . range , }) . collect () ,) . unwrap () ; match_finder . add_search_pattern (pattern . parse () . unwrap ()) . unwrap () ; let matched_strings : Vec < String > = match_finder . matches () . flattened () . matches . iter () . map (| m | m . matched_text ()) . collect () ; if matched_strings != expected && ! expected . is_empty () { print_match_debug_info (& match_finder , position . file_id , expected [0]) ; } assert_eq ! (matched_strings , expected) ; }) }
    };
}

assert_matches!();