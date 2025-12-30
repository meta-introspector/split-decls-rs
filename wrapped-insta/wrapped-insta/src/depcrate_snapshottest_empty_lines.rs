// Generated macro for test_empty_lines (function)
macro_rules! Depcrate_snapshottest_empty_lines {
() => {
// Module: crate::snapshot
// Provides: {"test_empty_lines"}
// Dependencies: {}
# [test] fn test_empty_lines () { assert_snapshot ! (r#"single line should fit on a single line"# , @ "single line should fit on a single line") ; assert_snapshot ! (r##"single line should fit on a single line, even if it's really really really really really really really really really long"## , @ "single line should fit on a single line, even if it's really really really really really really really really really long") ; assert_snapshot ! (r#"multiline content starting on first line

    final line
    "# , @ r"
    multiline content starting on first line

        final line
    ") ; assert_snapshot ! (r#"
    multiline content starting on second line

    final line
    "# , @ r"

    multiline content starting on second line

    final line
    ") ; }
};
}
