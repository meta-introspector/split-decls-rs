// SRC: ../rust/compiler/rustc_parse/src/parser/tokenstream/tests.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3 */
#[allow(rustc::symbol_intern_string_literal)]

use crate::rustc_complete::token::{self, IdentIsRaw};
/* AST_META: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1 */
use crate::rustc_complete::tokenstream::{TokenStream, TokenTree};
/* AST_META: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1 */
use crate::rustc_complete::{BytePos, Span, Symbol, create_default_session_globals_then};
/* AST_META: AST_ID=4 | TYPE=FUNCTION | NAME=string_to_ts | COMPLEXITY=2 | LINES=6 */

use crate::parser::tests::string_to_stream;

fn string_to_ts(string: &str) -> TokenStream {
    string_to_stream(string.to_owned())
}
/* AST_META: AST_ID=5 | TYPE=FUNCTION | NAME=sp | COMPLEXITY=2 | LINES=4 */

fn sp(a: u32, b: u32) -> Span {
    Span::with_root_ctxt(BytePos(a), BytePos(b))
}
/* AST_META: AST_ID=6 | TYPE=FUNCTION | NAME=cmp_token_stream | COMPLEXITY=2 | LINES=4 */

fn cmp_token_stream(a: &TokenStream, b: &TokenStream) -> bool {
    a.len() == b.len() && a.iter().zip(b.iter()).all(|(x, y)| x.eq_unspanned(y))
}
/* AST_META: AST_ID=7 | TYPE=FUNCTION | NAME=test_concat | COMPLEXITY=4 | LINES=15 */

#[test]
fn test_concat() {
    create_default_session_globals_then(|| {
        let test_res = string_to_ts("foo::bar::baz");
        let test_fst = string_to_ts("foo::bar");
        let test_snd = string_to_ts("::baz");
        let mut eq_res = TokenStream::default();
        eq_res.push_stream(test_fst);
        eq_res.push_stream(test_snd);
        assert_eq!(test_res.iter().count(), 5);
        assert_eq!(eq_res.iter().count(), 5);
        assert_eq!(cmp_token_stream(&test_res, &eq_res), true);
    })
}
/* AST_META: AST_ID=8 | TYPE=FUNCTION | NAME=test_to_from_bijection | COMPLEXITY=3 | LINES=9 */

#[test]
fn test_to_from_bijection() {
    create_default_session_globals_then(|| {
        let test_start = string_to_ts("foo::bar(baz)");
        let test_end = test_start.iter().cloned().collect();
        assert_eq!(test_start, test_end)
    })
}
/* AST_META: AST_ID=9 | TYPE=FUNCTION | NAME=test_eq_0 | COMPLEXITY=3 | LINES=9 */

#[test]
fn test_eq_0() {
    create_default_session_globals_then(|| {
        let test_res = string_to_ts("foo");
        let test_eqs = string_to_ts("foo");
        assert_eq!(test_res, test_eqs)
    })
}
/* AST_META: AST_ID=10 | TYPE=FUNCTION | NAME=test_eq_1 | COMPLEXITY=3 | LINES=9 */

#[test]
fn test_eq_1() {
    create_default_session_globals_then(|| {
        let test_res = string_to_ts("::bar::baz");
        let test_eqs = string_to_ts("::bar::baz");
        assert_eq!(test_res, test_eqs)
    })
}
/* AST_META: AST_ID=11 | TYPE=FUNCTION | NAME=test_eq_3 | COMPLEXITY=3 | LINES=9 */

#[test]
fn test_eq_3() {
    create_default_session_globals_then(|| {
        let test_res = string_to_ts("");
        let test_eqs = string_to_ts("");
        assert_eq!(test_res, test_eqs)
    })
}
/* AST_META: AST_ID=12 | TYPE=FUNCTION | NAME=test_diseq_0 | COMPLEXITY=3 | LINES=9 */

#[test]
fn test_diseq_0() {
    create_default_session_globals_then(|| {
        let test_res = string_to_ts("::bar::baz");
        let test_eqs = string_to_ts("bar::baz");
        assert_eq!(test_res == test_eqs, false)
    })
}
/* AST_META: AST_ID=13 | TYPE=FUNCTION | NAME=test_diseq_1 | COMPLEXITY=3 | LINES=9 */

#[test]
fn test_diseq_1() {
    create_default_session_globals_then(|| {
        let test_res = string_to_ts("(bar,baz)");
        let test_eqs = string_to_ts("bar,baz");
        assert_eq!(test_res == test_eqs, false)
    })
}
/* AST_META: AST_ID=14 | TYPE=FUNCTION | NAME=test_is_empty | COMPLEXITY=3 | LINES=14 */

#[test]
fn test_is_empty() {
    create_default_session_globals_then(|| {
        let test0 = TokenStream::default();
        let test1 =
            TokenStream::token_alone(token::Ident(Symbol::intern("a"), IdentIsRaw::No), sp(0, 1));
        let test2 = string_to_ts("foo(bar::baz)");

        assert_eq!(test0.is_empty(), true);
        assert_eq!(test1.is_empty(), false);
        assert_eq!(test2.is_empty(), false);
    })
}
/* AST_META: AST_ID=15 | TYPE=FUNCTION | NAME=test_dotdotdot | COMPLEXITY=3 | LINES=12 */

#[test]
fn test_dotdotdot() {
    create_default_session_globals_then(|| {
        let mut stream = TokenStream::default();
        stream.push_tree(TokenTree::token_joint(token::Dot, sp(0, 1)));
        stream.push_tree(TokenTree::token_joint(token::Dot, sp(1, 2)));
        stream.push_tree(TokenTree::token_alone(token::Dot, sp(2, 3)));
        assert!(cmp_token_stream(&stream, &string_to_ts("...")));
        assert_eq!(stream.iter().count(), 1);
    })
}