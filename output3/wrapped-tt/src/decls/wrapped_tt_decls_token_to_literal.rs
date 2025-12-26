use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub fn token_to_literal<S>(text: &str, span: S) -> Literal<S>
where
    S: Copy,
{
    use rustc_lexer::LiteralKind;
    let token = rustc_lexer::tokenize(text, rustc_lexer::FrontmatterAllowed::No).next_tuple();
    let Some((rustc_lexer::Token {
        kind: rustc_lexer::TokenKind::Literal { kind, suffix_start },
        ..
    },)) = token
    else {
        return Literal {
            span,
            symbol: Symbol::intern(text),
            kind: LitKind::Err(()),
            suffix: None,
        };
    };
    let (kind, start_offset, end_offset) = match kind {
        LiteralKind::Int { .. } => (LitKind::Integer, 0, 0),
        LiteralKind::Float { .. } => (LitKind::Float, 0, 0),
        LiteralKind::Char { terminated } => (LitKind::Char, 1, terminated as usize),
        LiteralKind::Byte { terminated } => (LitKind::Byte, 2, terminated as usize),
        LiteralKind::Str { terminated } => (LitKind::Str, 1, terminated as usize),
        LiteralKind::ByteStr { terminated } => (LitKind::ByteStr, 2, terminated as usize),
        LiteralKind::CStr { terminated } => (LitKind::CStr, 2, terminated as usize),
        LiteralKind::RawStr { n_hashes } => (
            LitKind::StrRaw(n_hashes.unwrap_or_default()),
            2 + n_hashes.unwrap_or_default() as usize,
            1 + n_hashes.unwrap_or_default() as usize,
        ),
        LiteralKind::RawByteStr { n_hashes } => (
            LitKind::ByteStrRaw(n_hashes.unwrap_or_default()),
            3 + n_hashes.unwrap_or_default() as usize,
            1 + n_hashes.unwrap_or_default() as usize,
        ),
        LiteralKind::RawCStr { n_hashes } => (
            LitKind::CStrRaw(n_hashes.unwrap_or_default()),
            3 + n_hashes.unwrap_or_default() as usize,
            1 + n_hashes.unwrap_or_default() as usize,
        ),
    };
    let (lit, suffix) = text.split_at(suffix_start as usize);
    let lit = &lit[start_offset..lit.len() - end_offset];
    let suffix = match suffix {
        "" | "_" => None,
        _ if !matches!(kind, LitKind::Integer | LitKind::Float | LitKind::Err(_)) => {
            return Literal {
                span,
                symbol: Symbol::intern(text),
                kind: LitKind::Err(()),
                suffix: None,
            };
        }
        suffix => Some(Symbol::intern(suffix)),
    };
    Literal {
        span,
        symbol: Symbol::intern(lit),
        kind,
        suffix,
    }
}
