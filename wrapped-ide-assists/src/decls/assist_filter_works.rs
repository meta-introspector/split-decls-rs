macro_rules! assist_filter_works {
    () => {
        # [test] fn assist_filter_works () { let (db , frange) = RootDatabase :: with_range (r#"
pub fn test_some_range(a: int) -> bool {
    if let 2..6 = $05$0 {
        true
    } else {
        false
    }
}
"# ,) ; { let mut cfg = TEST_CONFIG ; cfg . allowed = Some (vec ! [AssistKind :: Refactor]) ; let assists = assists (& db , & cfg , AssistResolveStrategy :: None , FileRange { file_id : frange . file_id . file_id (& db) , range : frange . range } ,) ; let expected = labels (& assists) ; expect ! [[r#"
            Extract into...
            Replace if let with match
        "#]] . assert_eq (& expected) ; } { let mut cfg = TEST_CONFIG ; cfg . allowed = Some (vec ! [AssistKind :: RefactorExtract]) ; let assists = assists (& db , & cfg , AssistResolveStrategy :: None , FileRange { file_id : frange . file_id . file_id (& db) , range : frange . range } ,) ; let expected = labels (& assists) ; expect ! [[r#"
            Extract into...
        "#]] . assert_eq (& expected) ; } { let mut cfg = TEST_CONFIG ; cfg . allowed = Some (vec ! [AssistKind :: QuickFix]) ; let assists = assists (& db , & cfg , AssistResolveStrategy :: None , FileRange { file_id : frange . file_id . file_id (& db) , range : frange . range } ,) ; let expected = labels (& assists) ; expect ! [[r#""#]] . assert_eq (& expected) ; } }
    };
}

assist_filter_works!()