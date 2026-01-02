// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_borrowck/src/prefixes.rs
// Error: expected square brackets
// Problematic line: line 11


use super::MirBorrowckCtxt;

pub(crate) trait IsPrefixOf<'tcx> {
    fn is_prefix_of(&self, other: PlaceRef<'tcx>) -> bool;
}

