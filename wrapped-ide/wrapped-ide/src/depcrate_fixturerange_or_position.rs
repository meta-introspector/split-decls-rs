// Generated macro for range_or_position (function)
macro_rules! Depcrate_fixturerange_or_position {
() => {
// Module: crate::fixture
// Provides: {"range_or_position"}
// Dependencies: {}
# [doc = " Creates analysis for a single file, returns range marked with a pair of $0 or a position marked with $0."] pub (crate) fn range_or_position (# [rust_analyzer :: rust_fixture] ra_fixture : & str ,) -> (Analysis , FileId , RangeOrOffset) { let mut host = AnalysisHost :: default () ; let change_fixture = ChangeFixture :: parse (& host . db , ra_fixture) ; host . db . enable_proc_attr_macros () ; host . db . apply_change (change_fixture . change) ; let (file_id , range_or_offset) = change_fixture . file_position . expect ("expected a marker ($0)") ; (host . analysis () , file_id . file_id (& host . db) , range_or_offset) }
};
}
