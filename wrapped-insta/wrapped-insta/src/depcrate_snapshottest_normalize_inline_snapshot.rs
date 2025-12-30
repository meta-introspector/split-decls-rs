// Generated macro for test_normalize_inline_snapshot (function)
macro_rules! Depcrate_snapshottest_normalize_inline_snapshot {
() => {
// Module: crate::snapshot
// Provides: {"test_normalize_inline_snapshot"}
// Dependencies: {}
# [test] fn test_normalize_inline_snapshot () { fn normalized_of_literal (snapshot : & str) -> String { normalize_inline (& TextSnapshotContents :: from_inline_literal (snapshot) . contents) } use similar_asserts :: assert_eq ; assert_eq ! (normalized_of_literal ("
   1
   2
") , "1
2") ; assert_eq ! (normalized_of_literal (r#"
            1
    2
    "#) , r"        1
2
") ; assert_eq ! (normalized_of_literal ("
            1
            2
    ") , r"1
2
") ; assert_eq ! (normalized_of_literal ("
   1
   2
") , "1
2") ; assert_eq ! (normalized_of_literal ("
        a
    ") , "        a") ; assert_eq ! (normalized_of_literal ("") , "") ; assert_eq ! (normalized_of_literal (r#"
    a
    b
c
    "#) , "    a
    b
c
    ") ; assert_eq ! (normalized_of_literal ("
a
    ") , "a") ; assert_eq ! (normalized_of_literal ("
    a") , "    a") ; }
};
}
