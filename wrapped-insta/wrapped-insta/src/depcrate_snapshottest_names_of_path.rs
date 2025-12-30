// Generated macro for test_names_of_path (function)
macro_rules! Depcrate_snapshottest_names_of_path {
() => {
// Module: crate::snapshot
// Provides: {"test_names_of_path"}
// Dependencies: {}
# [test] fn test_names_of_path () { assert_debug_snapshot ! (names_of_path (Path :: new ("/src/snapshots/insta_tests__tests__name_foo.snap")) , @ r#"
    (
        "name_foo",
        "insta_tests__tests",
    )
    "#) ; assert_debug_snapshot ! (names_of_path (Path :: new ("/src/snapshots/name_foo.snap")) , @ r#"
    (
        "name_foo",
        "",
    )
    "#) ; assert_debug_snapshot ! (names_of_path (Path :: new ("foo/src/snapshots/go1.20.5.snap")) , @ r#"
    (
        "go1.20.5",
        "",
    )
    "#) ; }
};
}
