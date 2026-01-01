// SRC: ../rust/compiler/rustc_resolve/src/rustdoc/tests.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3 */
use std::path::PathBuf;

use crate::rustc_complete::source_map::{FilePathMapping, SourceMap};
/* AST_META: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2 */
use crate::rustc_complete::symbol::sym;
use crate::rustc_complete::{BytePos, Span};
/* AST_META: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2 */

use super::{DocFragment, DocFragmentKind, source_span_for_markdown_range_inner};
/* AST_META: AST_ID=4 | TYPE=FUNCTION | NAME=single_backtick | COMPLEXITY=5 | LINES=22 */

#[test]
fn single_backtick() {
    let sm = SourceMap::new(FilePathMapping::empty());
    sm.new_source_file(PathBuf::from("foo.rs").into(), r#"#[doc = "`"] fn foo() {}"#.to_string());
    let (span, _) = source_span_for_markdown_range_inner(
        &sm,
        "`",
        &(0..1),
        &[DocFragment {
            span: Span::with_root_ctxt(BytePos(8), BytePos(11)),
            item_id: None,
            kind: DocFragmentKind::RawDoc,
            doc: sym::empty, // unused placeholder
            indent: 0,
            from_expansion: false,
        }],
    )
    .unwrap();
    assert_eq!(span.lo(), BytePos(9));
    assert_eq!(span.hi(), BytePos(10));
}
/* AST_META: AST_ID=5 | TYPE=FUNCTION | NAME=utf8 | COMPLEXITY=7 | LINES=23 */

#[test]
fn utf8() {
    // regression test for https://github.com/rust-lang/rust/issues/141665
    let sm = SourceMap::new(FilePathMapping::empty());
    sm.new_source_file(PathBuf::from("foo.rs").into(), r#"#[doc = "⚠"] fn foo() {}"#.to_string());
    let (span, _) = source_span_for_markdown_range_inner(
        &sm,
        "⚠",
        &(0..3),
        &[DocFragment {
            span: Span::with_root_ctxt(BytePos(8), BytePos(14)),
            item_id: None,
            kind: DocFragmentKind::RawDoc,
            doc: sym::empty, // unused placeholder
            indent: 0,
            from_expansion: false,
        }],
    )
    .unwrap();
    assert_eq!(span.lo(), BytePos(9));
    assert_eq!(span.hi(), BytePos(12));
}