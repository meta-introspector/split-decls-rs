// MINIMAL TEST CASE for parsing failure in: ../rust/library/proc_macro/src/quote.rs
// Error: expected square brackets
// Problematic line: line 7

//! This quasiquoter uses macros 2.0 hygiene to reliably access
//! items from `proc_macro`, to build a `proc_macro::TokenStream`.

use crate::{
    BitOr, Delimiter, Group, Ident, Literal, Punct, Spacing, Span, ToTokens, TokenStream, TokenTree,
};

