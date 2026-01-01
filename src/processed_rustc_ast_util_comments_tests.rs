// SRC: ../rust/compiler/rustc_ast/src/util/comments/tests.rs
/* AST_META: AST_ID=1 | TYPE=FUNCTION | NAME=test_block_doc_comment_1 | COMPLEXITY=3 | LINES=14 */
#[allow(rustc::symbol_intern_string_literal)]

use crate::rustc_complete::create_default_session_globals_then;

use super::*;

#[test]
fn test_block_doc_comment_1() {
    create_default_session_globals_then(|| {
        let comment = "\n * Test \n **  Test\n *   Test\n";
        let stripped = beautify_doc_string(Symbol::intern(comment), CommentKind::Block);
        assert_eq!(stripped.as_str(), " Test \n*  Test\n   Test");
    })
}
/* AST_META: AST_ID=2 | TYPE=FUNCTION | NAME=test_block_doc_comment_2 | COMPLEXITY=3 | LINES=9 */

#[test]
fn test_block_doc_comment_2() {
    create_default_session_globals_then(|| {
        let comment = "\n * Test\n *  Test\n";
        let stripped = beautify_doc_string(Symbol::intern(comment), CommentKind::Block);
        assert_eq!(stripped.as_str(), " Test\n  Test");
    })
}
/* AST_META: AST_ID=3 | TYPE=FUNCTION | NAME=test_block_doc_comment_3 | COMPLEXITY=3 | LINES=9 */

#[test]
fn test_block_doc_comment_3() {
    create_default_session_globals_then(|| {
        let comment = "\n let a: *i32;\n *a = 5;\n";
        let stripped = beautify_doc_string(Symbol::intern(comment), CommentKind::Block);
        assert_eq!(stripped.as_str(), "let a: *i32;\n*a = 5;");
    })
}
/* AST_META: AST_ID=4 | TYPE=FUNCTION | NAME=test_line_doc_comment | COMPLEXITY=4 | LINES=14 */

#[test]
fn test_line_doc_comment() {
    create_default_session_globals_then(|| {
        let stripped = beautify_doc_string(Symbol::intern(" test"), CommentKind::Line);
        assert_eq!(stripped.as_str(), " test");
        let stripped = beautify_doc_string(Symbol::intern("! test"), CommentKind::Line);
        assert_eq!(stripped.as_str(), "! test");
        let stripped = beautify_doc_string(Symbol::intern("test"), CommentKind::Line);
        assert_eq!(stripped.as_str(), "test");
        let stripped = beautify_doc_string(Symbol::intern("!test"), CommentKind::Line);
        assert_eq!(stripped.as_str(), "!test");
    })
}
/* AST_META: AST_ID=5 | TYPE=FUNCTION | NAME=test_doc_blocks | COMPLEXITY=4 | LINES=18 */

#[test]
fn test_doc_blocks() {
    create_default_session_globals_then(|| {
        let stripped =
            beautify_doc_string(Symbol::intern(" # Returns\n     *\n     "), CommentKind::Block);
        assert_eq!(stripped.as_str(), " # Returns\n\n");

        let stripped = beautify_doc_string(
            Symbol::intern("\n     * # Returns\n     *\n     "),
            CommentKind::Block,
        );
        assert_eq!(stripped.as_str(), " # Returns\n\n");

        let stripped = beautify_doc_string(Symbol::intern("\n *     a\n "), CommentKind::Block);
        assert_eq!(stripped.as_str(), "     a\n");
    })
}