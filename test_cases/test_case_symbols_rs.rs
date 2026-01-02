// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_macros/src/symbols.rs
// Error: expected square brackets
// Problematic line: line 35

use syn::punctuated::Punctuated;
use syn::{Expr, Ident, Lit, LitStr, Macro, Token, braced};

#[cfg(test)]
mod tests;

mod kw {
