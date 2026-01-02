// MINIMAL TEST CASE for parsing failure in: ../rust/library/alloc/src/vec/in_place_collect.rs
// Error: expected square brackets
// Problematic line: line 168

use super::{InPlaceDrop, InPlaceDstDataSrcBufDrop, SpecFromIter, SpecFromIterNested, Vec};
use crate::alloc::{Global, handle_alloc_error};

const fn in_place_collectible<DEST, SRC>(
    step_merge: Option<NonZero<usize>>,
    step_expand: Option<NonZero<usize>>,
) -> bool {
