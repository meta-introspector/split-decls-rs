// Generated macro for normalize_test_name (function)
macro_rules! Depcrate_test_dashboardnormalize_test_name {
() => {
// Module: crate::test_dashboard
// Provides: {"normalize_test_name"}
// Dependencies: {}
# [doc = " Compiletest tests start with `[suite] tests/[suite]/a/b/c...`."] # [doc = " Remove the `[suite] tests/[suite]/` prefix so that we can find the filesystem path."] # [doc = " Also normalizes path delimiters."] fn normalize_test_name (name : & str , suite_name : & str) -> String { let name = normalize_path_delimiters (name) ; let name = name . as_ref () ; let name = name . strip_prefix (& format ! ("[{suite_name}]")) . unwrap_or (name) . trim () ; let name = name . strip_prefix ("tests/") . unwrap_or (name) ; let name = name . strip_prefix (suite_name) . unwrap_or (name) ; name . trim_start_matches ("/") . to_string () }
};
}
