// Generated macro for test_snapshot_contents_hashes (function)
macro_rules! Depcrate_snapshottest_snapshot_contents_hashes {
() => {
// Module: crate::snapshot
// Provides: {"test_snapshot_contents_hashes"}
// Dependencies: {}
# [test] fn test_snapshot_contents_hashes () { assert_eq ! (TextSnapshotContents :: new ("a###b" . to_string () , TextSnapshotKind :: Inline) . to_inline ("") , r#""a###b""#) ; assert_eq ! (TextSnapshotContents :: new ("a\n\\###b" . to_string () , TextSnapshotKind :: Inline) . to_inline ("") , r#####"r"
a
\###b
""#####) ; }
};
}
