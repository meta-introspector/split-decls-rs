// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_ast/src/tokenstream.rs
// Error: expected square brackets
// Problematic line: line 24

use crate::token::{self, Delimiter, Token, TokenKind};
use crate::{AttrVec, Attribute};

/// Part of a `TokenStream`.
#[derive(Debug, Clone, PartialEq, Encodable, Decodable, HashStable_Generic)]
pub enum TokenTree {
    /// A single token. Should never be `OpenDelim` or `CloseDelim`, because
