// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_ast/src/util/literal.rs
// Error: expected square brackets
// Problematic line: line 5


use std::{ascii, fmt, str};

use rustc_literal_escaper::{
    MixedUnit, unescape_byte, unescape_byte_str, unescape_c_str, unescape_char, unescape_str,
};
use rustc_span::{ByteSymbol, Span, Symbol, kw, sym};
