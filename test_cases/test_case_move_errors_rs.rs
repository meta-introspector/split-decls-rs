// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_borrowck/src/diagnostics/move_errors.rs
// Error: expected square brackets
// Problematic line: line 22

use crate::diagnostics::{CapturedMessageOpt, DescribePlaceOpt, UseSpans};
use crate::prefixes::PrefixSet;

#[derive(Debug)]
pub(crate) enum IllegalMoveOriginKind<'tcx> {
    /// Illegal move due to attempt to move from behind a reference.
    BorrowedContent {
