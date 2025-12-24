use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Similar to `tokenstream::TokenTree`, except that `Sequence`, `MetaVar`, `MetaVarDecl`, and
/// `MetaVarExpr` are "first-class" token trees. Useful for parsing macros.
#[derive(Debug, PartialEq, Encodable, Decodable)]
pub enum TokenTree {
    /// A token. Unlike `tokenstream::TokenTree::Token` this lacks a `Spacing`.
    /// See the comments about `Spacing` in the `transcribe` function.
    Token(Token),
    /// A delimited sequence, e.g. `($e:expr)` (RHS) or `{ $e }` (LHS).
    Delimited(DelimSpan, DelimSpacing, Delimited),
    /// A kleene-style repetition sequence, e.g. `$($e:expr)*` (RHS) or `$($e),*` (LHS).
    Sequence(DelimSpan, SequenceRepetition),
    /// e.g., `$var`. The span covers the leading dollar and the ident. (The span within the ident
    /// only covers the ident, e.g. `var`.)
    MetaVar(Span, Ident),
    /// e.g., `$var:expr`. Only appears on the LHS.
    MetaVarDecl {
        span: Span,
        /// Name to bind.
        name: Ident,
        /// The fragment specifier.
        kind: NonterminalKind,
    },
    /// A meta-variable expression inside ${...}.
    MetaVarExpr(DelimSpan, MetaVarExpr),
}
