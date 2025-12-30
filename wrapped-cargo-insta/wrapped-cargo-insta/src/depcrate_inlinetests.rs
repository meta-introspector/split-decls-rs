// Generated macro for tests (module)
macro_rules! Depcrate_inlinetests {
() => {
// Module: crate::inline
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use insta :: assert_debug_snapshot ; use super :: * ; use std :: path :: PathBuf ; # [test] fn test_find_snapshot_macro () { let content = r######"
use insta::assert_snapshot;

fn test_function() {
    assert_snapshot!("test\ntest", @r###"
    test
    test
    "###);
}
"###### ; let file_patcher = FilePatcher { filename : PathBuf :: new () , lines : content . lines () . map (String :: from) . collect () , source : syn :: parse_file (content) . unwrap () , inline_snapshots : vec ! [] , } ; let snapshot = file_patcher . find_snapshot_macro (5) . unwrap () ; let snapshot_content : Vec < String > = file_patcher . lines [snapshot . start . 0 ..= snapshot . end . 0] . to_vec () ; assert_debug_snapshot ! (snapshot_content , @ r####"
        [
            "    assert_snapshot!(\"test\\ntest\", @r###\"",
            "    test",
            "    test",
            "    \"###);",
        ]
        "####) ; assert_debug_snapshot ! (snapshot . indentation , @ r#""    ""#) ; } # [test] fn test_find_snapshot_macro_with_tabs () { let content = r######"
use insta::assert_snapshot;

fn test_function() {
	assert_snapshot!("test\ntest", @r###"
	test
	test
	"###);
	// visitor shouldn't panic because of macro at column > start_line_len
	                                       assert_snapshot!("", @"");
}
"###### ; let file_patcher = FilePatcher { filename : PathBuf :: new () , lines : content . lines () . map (String :: from) . collect () , source : syn :: parse_file (content) . unwrap () , inline_snapshots : vec ! [] , } ; let snapshot = file_patcher . find_snapshot_macro (5) . unwrap () ; let snapshot_content : Vec < String > = file_patcher . lines [snapshot . start . 0 ..= snapshot . end . 0] . to_vec () ; assert_debug_snapshot ! (snapshot_content , @ r####"
        [
            "\tassert_snapshot!(\"test\\ntest\", @r###\"",
            "\ttest",
            "\ttest",
            "\t\"###);",
        ]
        "####) ; assert_debug_snapshot ! (snapshot . indentation , @ r#""\t""#) ; } # [test] fn test_find_snapshot_macro_within_allow_duplicates () { let content = r######"
fn test_function() {
    insta::allow_duplicates! {
        for x in 0..10 {
            insta::assert_snapshot!("foo", @"foo"); // 5
            insta::assert_snapshot!("bar", @"bar"); // 6
        }
    }
}
"###### ; let file_patcher = FilePatcher { filename : PathBuf :: new () , lines : content . lines () . map (String :: from) . collect () , source : syn :: parse_file (content) . unwrap () , inline_snapshots : vec ! [] , } ; let snapshot5 = file_patcher . find_snapshot_macro (5) . unwrap () ; let snapshot6 = file_patcher . find_snapshot_macro (6) . unwrap () ; let snapshot_content5 = file_patcher . lines [snapshot5 . start . 0 ..= snapshot5 . end . 0] . to_vec () ; let snapshot_content6 = file_patcher . lines [snapshot6 . start . 0 ..= snapshot6 . end . 0] . to_vec () ; assert_debug_snapshot ! (snapshot_content5 , @ r#"
        [
            "            insta::assert_snapshot!(\"foo\", @\"foo\"); // 5",
        ]
        "#) ; assert_debug_snapshot ! (snapshot_content6 , @ r#"
        [
            "            insta::assert_snapshot!(\"bar\", @\"bar\"); // 6",
        ]
        "#) ; assert_debug_snapshot ! (snapshot5 . indentation , @ r#""            ""#) ; assert_debug_snapshot ! (snapshot6 . indentation , @ r#""            ""#) ; } }
};
}
