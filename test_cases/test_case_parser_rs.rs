// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_attr_parsing/src/parser.rs
// Error: expected square brackets
// Problematic line: line 23

use thin_vec::ThinVec;

use crate::ShouldEmit;
use crate::session_diagnostics::{
    InvalidMetaItem, InvalidMetaItemQuoteIdentSugg, InvalidMetaItemRemoveNegSugg, MetaBadDelim,
    MetaBadDelimSugg, SuffixedLiteralInAttribute,
};
