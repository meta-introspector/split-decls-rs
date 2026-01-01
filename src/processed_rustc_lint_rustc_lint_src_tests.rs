// SRC: ../rust/compiler/rustc_lint/src/tests.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3 */
#[allow(rustc::symbol_intern_string_literal)]

use crate::rustc_complete::{Symbol, create_default_session_globals_then};
/* AST_META: AST_ID=2 | TYPE=FUNCTION | NAME=parse_lint_no_tool | COMPLEXITY=3 | LINES=9 */

use crate::levels::parse_lint_and_tool_name;

#[test]
fn parse_lint_no_tool() {
    create_default_session_globals_then(|| {
        assert_eq!(parse_lint_and_tool_name("foo"), (None, "foo"))
    });
}
/* AST_META: AST_ID=3 | TYPE=FUNCTION | NAME=parse_lint_with_tool | COMPLEXITY=3 | LINES=7 */

#[test]
fn parse_lint_with_tool() {
    create_default_session_globals_then(|| {
        assert_eq!(parse_lint_and_tool_name("clippy::foo"), (Some(Symbol::intern("clippy")), "foo"))
    });
}
/* AST_META: AST_ID=4 | TYPE=FUNCTION | NAME=parse_lint_multiple_path | COMPLEXITY=3 | LINES=10 */

#[test]
fn parse_lint_multiple_path() {
    create_default_session_globals_then(|| {
        assert_eq!(
            parse_lint_and_tool_name("clippy::foo::bar"),
            (Some(Symbol::intern("clippy")), "foo::bar")
        )
    });
}