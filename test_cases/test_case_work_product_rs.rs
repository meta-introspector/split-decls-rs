// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_incremental/src/persist/work_product.rs
// Error: expected square brackets
// Problematic line: line 17

use crate::errors;
use crate::persist::fs::*;

/// Copies a CGU work product to the incremental compilation directory, so next compilation can
/// find and reuse it.
pub fn copy_cgu_workproduct_to_incr_comp_cache_dir(
    sess: &Session,
