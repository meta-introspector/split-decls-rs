// Generated macro for annotations (function)
macro_rules! Depcrate_fixtureannotations {
() => {
// Module: crate::fixture
// Provides: {"annotations"}
// Dependencies: {}
# [doc = " Creates analysis from a multi-file fixture, returns positions marked with $0."] pub (crate) fn annotations (# [rust_analyzer :: rust_fixture] ra_fixture : & str ,) -> (Analysis , FilePosition , Vec < (FileRange , String) >) { let mut host = AnalysisHost :: default () ; let change_fixture = ChangeFixture :: parse (& host . db , ra_fixture) ; host . db . enable_proc_attr_macros () ; host . db . apply_change (change_fixture . change) ; let (file_id , range_or_offset) = change_fixture . file_position . expect ("expected a marker ($0)") ; let offset = range_or_offset . expect_offset () ; let db = & host . db ; let annotations = change_fixture . files . iter () . flat_map (| & file_id | { let file_text = host . analysis () . file_text (file_id . file_id (& host . db)) . unwrap () ; let annotations = extract_annotations (& file_text) ; annotations . into_iter () . map (move | (range , data) | (FileRange { file_id : file_id . file_id (db) , range } , data)) }) . collect () ; (host . analysis () , FilePosition { file_id : file_id . file_id (& host . db) , offset } , annotations) }
};
}
