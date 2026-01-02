// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_metadata/src/rmeta/decoder.rs
// Error: unexpected token, expected `;`
// Error type: unexpected_token
// Sample #1 of 3
// Problematic line: line 35

use rustc_session::config::TargetModifier;
use rustc_session::cstore::{CrateSource, ExternCrate};
use rustc_span::hygiene::HygieneDecodeContext;
use rustc_span::{
    BytePos, ByteSymbol, DUMMY_SP, Pos, SpanData, SpanDecoder, Symbol, SyntaxContext, kw,
};
use tracing::debug;
