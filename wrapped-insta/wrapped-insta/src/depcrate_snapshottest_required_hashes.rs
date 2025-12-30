// Generated macro for test_required_hashes (function)
macro_rules! Depcrate_snapshottest_required_hashes {
() => {
// Module: crate::snapshot
// Provides: {"test_required_hashes"}
// Dependencies: {}
# [test] fn test_required_hashes () { assert_snapshot ! (required_hashes ("") , @ "0") ; assert_snapshot ! (required_hashes ("Hello, world!") , @ "0") ; assert_snapshot ! (required_hashes ("\"\"") , @ "1") ; assert_snapshot ! (required_hashes ("##") , @ "0") ; assert_snapshot ! (required_hashes ("\"#\"#") , @ "2") ; assert_snapshot ! (required_hashes (r##""#"##) , @ "2") ; assert_snapshot ! (required_hashes (r######"foo ""##### bar "###" baz"######) , @ "6") ; assert_snapshot ! (required_hashes ("\"\"\"") , @ "1") ; assert_snapshot ! (required_hashes ("####") , @ "0") ; assert_snapshot ! (required_hashes (r###"\"\"##\"\""###) , @ "3") ; assert_snapshot ! (required_hashes (r###"r"#"Raw string"#""###) , @ "2") ; }
};
}
