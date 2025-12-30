// Generated macro for position (function)
macro_rules! Depcrate_fixtureposition {
() => {
// Module: crate::fixture
// Provides: {"position"}
// Dependencies: {}
# [doc = " Creates analysis from a multi-file fixture, returns positions marked with $0."] pub (crate) fn position (# [rust_analyzer :: rust_fixture] ra_fixture : & str ,) -> (Analysis , FilePosition) { let mut host = AnalysisHost :: default () ; let change_fixture = ChangeFixture :: parse (& host . db , ra_fixture) ; host . db . enable_proc_attr_macros () ; host . db . apply_change (change_fixture . change) ; let (file_id , range_or_offset) = change_fixture . file_position . expect ("expected a marker ($0)") ; let offset = range_or_offset . expect_offset () ; (host . analysis () , FilePosition { file_id : file_id . file_id (& host . db) , offset }) }
};
}
