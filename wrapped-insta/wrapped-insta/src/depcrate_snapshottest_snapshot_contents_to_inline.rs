// Generated macro for test_snapshot_contents_to_inline (function)
macro_rules! Depcrate_snapshottest_snapshot_contents_to_inline {
() => {
// Module: crate::snapshot
// Provides: {"test_snapshot_contents_to_inline"}
// Dependencies: {}
# [test] fn test_snapshot_contents_to_inline () { use similar_asserts :: assert_eq ; let snapshot_contents = TextSnapshotContents :: new ("testing" . to_string () , TextSnapshotKind :: Inline) ; assert_eq ! (snapshot_contents . to_inline ("") , r#""testing""#) ; assert_eq ! (TextSnapshotContents :: new ("\na\nb" . to_string () , TextSnapshotKind :: Inline) . to_inline ("") , r##"r"

a
b
""##) ; assert_eq ! (TextSnapshotContents :: new ("a\nb" . to_string () , TextSnapshotKind :: Inline) . to_inline ("    ") , r##"r"
    a
    b
    ""##) ; assert_eq ! (TextSnapshotContents :: new ("\n    a\n    b" . to_string () , TextSnapshotKind :: Inline) . to_inline ("") , r##"r"

a
b
""##) ; assert_eq ! (TextSnapshotContents :: new ("\na\n\nb" . to_string () , TextSnapshotKind :: Inline) . to_inline ("    ") , r##"r"

    a

    b
    ""##) ; assert_eq ! (TextSnapshotContents :: new ("ab
    " . to_string () , TextSnapshotKind :: Inline) . to_inline ("") , r#""ab""#) ; assert_eq ! (TextSnapshotContents :: new ("    ab
    " . to_string () , TextSnapshotKind :: Inline) . to_inline ("") , r##""    ab""##) ; assert_eq ! (TextSnapshotContents :: new ("\n    ab\n" . to_string () , TextSnapshotKind :: Inline) . to_inline ("") , r##"r"

ab
""##) ; assert_eq ! (TextSnapshotContents :: new ("ab" . to_string () , TextSnapshotKind :: Inline) . to_inline ("") , r#""ab""#) ; assert_eq ! (TextSnapshotContents :: new ("a\tb" . to_string () , TextSnapshotKind :: Inline) . to_inline ("") , r##""a	b""##) ; assert_eq ! (TextSnapshotContents :: new ("a\t\nb" . to_string () , TextSnapshotKind :: Inline) . to_inline ("") , r##"r"
a	
b
""##) ; assert_eq ! (TextSnapshotContents :: new ("a\rb" . to_string () , TextSnapshotKind :: Inline) . to_inline ("") , r##""a\rb""##) ; assert_eq ! (TextSnapshotContents :: new ("a\0b" . to_string () , TextSnapshotKind :: Inline) . to_inline ("") , r##""a\0b""##) ; assert_eq ! (TextSnapshotContents :: new ("a\u{FFFD}b" . to_string () , TextSnapshotKind :: Inline) . to_inline ("") , r##""a�b""##) ; }
};
}
