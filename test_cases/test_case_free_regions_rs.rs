// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_infer/src/infer/free_regions.rs
// Error: expected square brackets
// Problematic line: line 10

use rustc_middle::ty::{Region, TyCtxt};
use tracing::debug;

/// Combines a `FreeRegionMap` and a `TyCtxt`.
///
/// This stuff is a bit convoluted and should be refactored, but as we
/// transition to NLL, it'll all go away anyhow.
