macro_rules! deps {
    () => {
        MatchFinder!();
        SsrRule!();
    };
}

macro_rules! assert_ssr_transforms {
    () => {
        deps!();
        fn assert_ssr_transforms (rules : & [& str] , input : & str , expected : Expect) { let (db , position , selections) = single_file (input) ; hir :: attach_db (& db , | | { let position = ide_db :: FilePosition { file_id : position . file_id . file_id (& db) , offset : position . offset , } ; let mut match_finder = MatchFinder :: in_context (& db , position , selections . into_iter () . map (| selection | ide_db :: FileRange { file_id : selection . file_id . file_id (& db) , range : selection . range , }) . collect () ,) . unwrap () ; for rule in rules { let rule : SsrRule = rule . parse () . unwrap () ; match_finder . add_rule (rule) . unwrap () ; } let edits = match_finder . edits () ; if edits . is_empty () { panic ! ("No edits were made") ; } let mut actual = db . file_text (position . file_id) . text (& db) . to_string () ; edits [& position . file_id] . apply (& mut actual) ; expected . assert_eq (& actual) ; }) }
    };
}

assert_ssr_transforms!()