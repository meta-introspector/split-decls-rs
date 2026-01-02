// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_pattern_analysis/src/constructor.rs
// Error: expected square brackets
// Problematic line: line 193

use self::SliceKind::*;
use crate::PatCx;

/// Whether we have seen a constructor in the column or not.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Presence {
    Unseen,
