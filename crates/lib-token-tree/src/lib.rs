use rustc_ast::token::{Delimiter, NonterminalKind, Token, TokenKind};
use rustc_ast::tokenstream::{DelimSpacing, DelimSpan};
use rustc_macros::{Decodable, Encodable};
use rustc_span::{Ident, Span};
use lib_kleene_op::KleeneOp;

use lib_metavar_expr::MetaVarExpr;

/// Contains the sub-token-trees of a "delimited" token tree such as `(a b c)`.
/// The delimiters are not represented explicitly in the `tts` vector.
#[derive(PartialEq, Encodable, Decodable, Debug)]
pub struct Delimited {
    pub delim: Delimiter,
    /// FIXME: #67062 has details about why this is sub-optimal.
    pub tts: Vec<TokenTree>,
}

#[derive(PartialEq, Encodable, Decodable, Debug)]
pub struct SequenceRepetition {
    /// The sequence of token trees
    pub tts: Vec<TokenTree>,
    /// The optional separator
    pub separator: Option<Token>,
    /// Whether the sequence can be repeated zero (*), or one or more times (+)
    pub kleene: KleeneToken,
    /// The number of `Match`s that appear in the sequence (and subsequences)
    pub num_captures: usize,
}

#[derive(Clone, PartialEq, Encodable, Decodable, Debug, Copy)]
pub struct KleeneToken {
    pub span: Span,
    pub op: KleeneOp,
}

impl KleeneToken {
    pub fn new(op: KleeneOp, span: Span) -> KleeneToken {
        KleeneToken { span, op }
    }
}

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

impl TokenTree {
    /// Returns `true` if the given token tree is delimited.
    pub fn is_delimited(&self) -> bool {
        matches!(*self, TokenTree::Delimited(..))
    }

    /// Returns `true` if the given token tree is a token of the given kind.
    pub fn is_token(&self, expected_kind: &TokenKind) -> bool {
        match self {
            TokenTree::Token(Token { kind: actual_kind, .. }) => actual_kind == expected_kind,
            _ => false,
        }
    }

    /// Retrieves the `TokenTree`'s span.
    pub fn span(&self) -> Span {
        match *self {
            TokenTree::Token(Token { span, .. })
            | TokenTree::MetaVar(span, _)
            | TokenTree::MetaVarDecl { span, .. } => span,
            TokenTree::Delimited(span, ..)
            | TokenTree::MetaVarExpr(span, _)
            | TokenTree::Sequence(span, _) => span.entire(),
        }
    }

    pub fn token(kind: TokenKind, span: Span) -> TokenTree {
        TokenTree::Token(Token::new(kind, span))
    }
}