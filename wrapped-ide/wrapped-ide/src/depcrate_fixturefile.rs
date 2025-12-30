// Generated macro for file (function)
macro_rules! Depcrate_fixturefile {
() => {
// Module: crate::fixture
// Provides: {"file"}
// Dependencies: {}
# [doc = " Creates analysis for a single file."] pub (crate) fn file (# [rust_analyzer :: rust_fixture] ra_fixture : & str) -> (Analysis , FileId) { let mut host = AnalysisHost :: default () ; let change_fixture = ChangeFixture :: parse (& host . db , ra_fixture) ; host . db . enable_proc_attr_macros () ; host . db . apply_change (change_fixture . change) ; (host . analysis () , change_fixture . files [0] . file_id (& host . db)) }
};
}
