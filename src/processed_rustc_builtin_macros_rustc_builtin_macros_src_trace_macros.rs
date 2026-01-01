// SRC: ../rust/compiler/rustc_builtin_macros/src/trace_macros.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1 */
use crate::rustc_complete::tokenstream::{TokenStream, TokenTree};
/* AST_META: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1 */
use crate::rustc_expand::base::{DummyResult, ExpandResult, ExtCtxt, MacroExpanderResult};
/* AST_META: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1 */
use crate::rustc_complete::{Span, kw};
/* AST_META: AST_ID=4 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=17 | LINES=27 */

use crate::errors;

pub(crate) fn expand_trace_macros(
    cx: &mut ExtCtxt<'_>,
    sp: Span,
    tt: TokenStream,
) -> MacroExpanderResult<'static> {
    let mut iter = tt.iter();
    let mut err = false;
    let value = match iter.next() {
        Some(TokenTree::Token(token, _)) if token.is_keyword(kw::True) => true,
        Some(TokenTree::Token(token, _)) if token.is_keyword(kw::False) => false,
        _ => {
            err = true;
            false
        }
    };
    err |= iter.next().is_some();
    if err {
        cx.dcx().emit_err(errors::TraceMacros { span: sp });
    } else {
        cx.set_trace_macros(value);
    }

    ExpandResult::Ready(DummyResult::any_valid(sp))
}