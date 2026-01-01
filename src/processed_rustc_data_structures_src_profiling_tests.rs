// SRC: ../rust/compiler/rustc_data_structures/src/profiling/tests.rs
/* AST_META: AST_ID=1 | TYPE=FUNCTION | NAME=with_rss | COMPLEXITY=4 | LINES=9 */
use super::JsonTimePassesEntry;

#[test]
fn with_rss() {
    let entry =
        JsonTimePassesEntry { pass: "typeck", time: 56.1, start_rss: Some(10), end_rss: Some(20) };

    assert_eq!(entry.to_string(), r#"{"pass":"typeck","time":56.1,"rss_start":10,"rss_end":20}"#)
}
/* AST_META: AST_ID=2 | TYPE=FUNCTION | NAME=no_rss | COMPLEXITY=4 | LINES=10 */

#[test]
fn no_rss() {
    let entry = JsonTimePassesEntry { pass: "typeck", time: 56.1, start_rss: None, end_rss: None };

    assert_eq!(
        entry.to_string(),
        r#"{"pass":"typeck","time":56.1,"rss_start":null,"rss_end":null}"#
    )
}