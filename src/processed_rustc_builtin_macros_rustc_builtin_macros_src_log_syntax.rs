// SRC: ../rust/compiler/rustc_builtin_macros/src/log_syntax.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3 */
use crate::rustc_complete::tokenstream::TokenStream;
use rustc_ast_pretty::pprust;
use crate::rustc_expand::base::{DummyResult, ExpandResult, ExtCtxt, MacroExpanderResult};
/* AST_META: AST_ID=2 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=3 | LINES=11 */

pub(crate) fn expand_log_syntax<'cx>(
    _cx: &'cx mut ExtCtxt<'_>,
    sp: crate::rustc_span::Span,
    tts: TokenStream,
) -> MacroExpanderResult<'cx> {
    println!("{}", pprust::tts_to_string(&tts));

    // any so that `log_syntax` can be invoked as an expression and item.
    ExpandResult::Ready(DummyResult::any_valid(sp))
}