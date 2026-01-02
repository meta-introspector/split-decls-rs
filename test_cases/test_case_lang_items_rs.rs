// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_passes/src/lang_items.rs
// Error: expected square brackets
// Problematic line: line 21

use rustc_session::cstore::ExternCrate;
use rustc_span::Span;

use crate::errors::{
    DuplicateLangItem, IncorrectCrateType, IncorrectTarget, LangItemOnIncorrectTarget,
    UnknownLangItem,
};
