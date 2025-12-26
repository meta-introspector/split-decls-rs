use std::fmt::Display;

use rustc_ast::token::{NonterminalKind, Token};
use rustc_span::{Ident, Span};
use lib_kleene_op::KleeneOp;
use lib_token_tree::TokenTree;

/// A unit within a matcher that a `MatcherPos` can refer to. Similar to (and derived from)
/// `mbe::TokenTree`, but designed specifically for fast and easy traversal during matching.
/// Notable differences to `mbe::TokenTree`:
/// - It is non-recursive, i.e. there is no nesting.
/// - The end pieces of each sequence (the separator, if present, and the Kleene op) are
///   represented explicitly, as is the very end of the matcher.
///
/// This means a matcher can be represented by `&[MatcherLoc]`, and traversal mostly involves
/// simply incrementing the current matcher position index by one.
#[derive(Debug, PartialEq, Clone)]
pub enum MatcherLoc {
    Token {
        token: Token,
    },
    Delimited,
    Sequence {
        op: KleeneOp,
        num_metavar_decls: usize,
        idx_first_after: usize,
        next_metavar: usize,
        seq_depth: usize,
    },
    SequenceKleeneOpNoSep {
        op: KleeneOp,
        idx_first: usize,
    },
    SequenceSep {
        separator: Token,
    },
    SequenceKleeneOpAfterSep {
        idx_first: usize,
    },
    MetaVarDecl {
        span: Span,
        bind: Ident,
        kind: NonterminalKind,
        next_metavar: usize,
        seq_depth: usize,
    },
    Eof,
}

impl MatcherLoc {
    pub fn span(&self) -> Option<Span> {
        match self {
            MatcherLoc::Token { token } => Some(token.span),
            MatcherLoc::Delimited => None,
            MatcherLoc::Sequence { .. } => None,
            MatcherLoc::SequenceKleeneOpNoSep { .. } => None,
            MatcherLoc::SequenceSep { .. } => None,
            MatcherLoc::SequenceKleeneOpAfterSep { .. } => None,
            MatcherLoc::MetaVarDecl { span, .. } => Some(*span),
            MatcherLoc::Eof => None,
        }
    }
}

impl Display for MatcherLoc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MatcherLoc::Token { token } | MatcherLoc::SequenceSep { separator: token } => {
                // TODO: Replace with `token_descr` once available
                write!(f, "{:?}", token)
            }
            MatcherLoc::MetaVarDecl { bind, kind, .. } => {
                write!(f, "meta-variable `${bind}:{kind}`")
            }
            MatcherLoc::Eof => f.write_str("end of macro"),

            // These are not printed in the diagnostic
            MatcherLoc::Delimited => f.write_str("delimiter"),
            MatcherLoc::Sequence { .. } => f.write_str("sequence start"),
            MatcherLoc::SequenceKleeneOpNoSep { .. } => f.write_str("sequence end"),
            MatcherLoc::SequenceKleeneOpAfterSep { .. } => f.write_str("sequence end"),
        }
    }
}

pub fn compute_locs(matcher: &[TokenTree]) -> Vec<MatcherLoc> {
    fn inner(
        tts: &[TokenTree],
        locs: &mut Vec<MatcherLoc>,
        next_metavar: &mut usize,
        seq_depth: usize,
    ) {
        for tt in tts {
            match tt {
                TokenTree::Token(token) => {
                    locs.push(MatcherLoc::Token { token: *token });
                }
                TokenTree::Delimited(span, _, delimited) => {
                    let open_token = Token::new(delimited.delim.as_open_token_kind(), span.open);
                    let close_token = Token::new(delimited.delim.as_close_token_kind(), span.close);

                    locs.push(MatcherLoc::Delimited);
                    locs.push(MatcherLoc::Token { token: open_token });
                    inner(&delimited.tts, locs, next_metavar, seq_depth);
                    locs.push(MatcherLoc::Token { token: close_token });
                }
                TokenTree::Sequence(_, seq) => {
                    let dummy = MatcherLoc::Eof;
                    locs.push(dummy);

                    let next_metavar_orig = *next_metavar;
                    let op = seq.kleene.op;
                    let idx_first = locs.len();
                    let idx_seq = idx_first - 1;
                    inner(&seq.tts, locs, next_metavar, seq_depth + 1);

                    if let Some(separator) = &seq.separator {
                        locs.push(MatcherLoc::SequenceSep { separator: separator.clone() });
                        locs.push(MatcherLoc::SequenceKleeneOpAfterSep { idx_first });
                    } else {
                        locs.push(MatcherLoc::SequenceKleeneOpNoSep { op, idx_first });
                    }

                    locs[idx_seq] = MatcherLoc::Sequence {
                        op,
                        num_metavar_decls: seq.num_captures,
                        idx_first_after: locs.len(),
                        next_metavar: next_metavar_orig,
                        seq_depth,
                    };
                }
                &TokenTree::MetaVarDecl { span, name: bind, kind } => {
                    locs.push(MatcherLoc::MetaVarDecl {
                        span,
                        bind,
                        kind,
                        next_metavar: *next_metavar,
                        seq_depth,
                    });
                    *next_metavar += 1;
                }
                TokenTree::MetaVar(..) | TokenTree::MetaVarExpr(..) => unreachable!(),
            }
        }
    }

    let mut locs = vec![];
    let mut next_metavar = 0;
    inner(matcher, &mut locs, &mut next_metavar, /* seq_depth */ 0);

    locs.push(MatcherLoc::Eof);

    locs
}