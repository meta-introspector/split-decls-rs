// SRC: ../rust/compiler/rustc_expand/src/mbe.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=3 | LINES=16 */
// This module implements declarative macros: old `macro_rules` and the newer
// `macro`. Declarative macros are also known as "macro by example", and that's
// why we call this module `mbe`. For external documentation, prefer the
// official terminology: "declarative macros".



use metavar_expr::MetaVarExpr;
use crate::rustc_complete::token::{Delimiter, NonterminalKind, Token, TokenKind};
/* AST_META: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1 */
use crate::rustc_complete::tokenstream::{DelimSpacing, DelimSpan};
/* AST_META: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1 */
use rustc_macros::{Decodable, Encodable};
/* AST_META: AST_ID=4 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1 */
use crate::rustc_complete::{Ident, Span};
/* AST_META: AST_ID=5 | TYPE=STRUCT | NAME=Delimited | COMPLEXITY=2 | LINES=9 */

/// Contains the sub-token-trees of a "delimited" token tree such as `(a b c)`.
/// The delimiters are not represented explicitly in the `tts` vector.
#[derive(PartialEq, Encodable, Decodable, Debug)]
struct Delimited {
    delim: Delimiter,
    /// FIXME: #67062 has details about why this is sub-optimal.
    tts: Vec<TokenTree>,
}
/* AST_META: AST_ID=6 | TYPE=STRUCT | NAME=SequenceRepetition | COMPLEXITY=2 | LINES=12 */

#[derive(PartialEq, Encodable, Decodable, Debug)]
struct SequenceRepetition {
    /// The sequence of token trees
    tts: Vec<TokenTree>,
    /// The optional separator
    separator: Option<Token>,
    /// Whether the sequence can be repeated zero (*), or one or more times (+)
    kleene: KleeneToken,
    /// The number of `Match`s that appear in the sequence (and subsequences)
    num_captures: usize,
}
/* AST_META: AST_ID=7 | TYPE=STRUCT | NAME=KleeneToken | COMPLEXITY=2 | LINES=6 */

#[derive(Clone, PartialEq, Encodable, Decodable, Debug, Copy)]
struct KleeneToken {
    span: Span,
    op: KleeneOp,
}
/* AST_META: AST_ID=8 | TYPE=FUNCTION | NAME=new | COMPLEXITY=4 | LINES=6 */

impl KleeneToken {
    fn new(op: KleeneOp, span: Span) -> KleeneToken {
        KleeneToken { span, op }
    }
}
/* AST_META: AST_ID=9 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=10 | LINES=12 */

/// A Kleene-style [repetition operator](https://en.wikipedia.org/wiki/Kleene_star)
/// for token sequences.
#[derive(Clone, PartialEq, Encodable, Decodable, Debug, Copy)]
pub(crate) enum KleeneOp {
    /// Kleene star (`*`) for zero or more repetitions
    ZeroOrMore,
    /// Kleene plus (`+`) for one or more repetitions
    OneOrMore,
    /// Kleene optional (`?`) for zero or one repetitions
    ZeroOrOne,
}
/* AST_META: AST_ID=10 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=9 | LINES=26 */

/// Similar to `tokenstream::TokenTree`, except that `Sequence`, `MetaVar`, `MetaVarDecl`, and
/// `MetaVarExpr` are "first-class" token trees. Useful for parsing macros.
#[derive(Debug, PartialEq, Encodable, Decodable)]
enum TokenTree {
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
    /// A meta-variable expression inside `${...}`.
    MetaVarExpr(DelimSpan, MetaVarExpr),
}
/* AST_META: AST_ID=11 | TYPE=FUNCTION | NAME=is_delimited | COMPLEXITY=22 | LINES=31 */

impl TokenTree {
    /// Returns `true` if the given token tree is delimited.
    fn is_delimited(&self) -> bool {
        matches!(*self, TokenTree::Delimited(..))
    }

    /// Returns `true` if the given token tree is a token of the given kind.
    fn is_token(&self, expected_kind: &TokenKind) -> bool {
        match self {
            TokenTree::Token(Token { kind: actual_kind, .. }) => actual_kind == expected_kind,
            _ => false,
        }
    }

    /// Retrieves the `TokenTree`'s span.
    fn span(&self) -> Span {
        match *self {
            TokenTree::Token(Token { span, .. })
            | TokenTree::MetaVar(span, _)
            | TokenTree::MetaVarDecl { span, .. } => span,
            TokenTree::Delimited(span, ..)
            | TokenTree::MetaVarExpr(span, _)
            | TokenTree::Sequence(span, _) => span.entire(),
        }
    }

    fn token(kind: TokenKind, span: Span) -> TokenTree {
        TokenTree::Token(Token::new(kind, span))
    }
}