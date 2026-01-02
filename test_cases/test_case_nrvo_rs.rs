// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_mir_transform/src/nrvo.rs
// Error: expected square brackets
// Problematic line: line 11

use rustc_middle::ty::TyCtxt;
use tracing::{debug, trace};

/// This pass looks for MIR that always copies the same local into the return place and eliminates
/// the copy by renaming all uses of that local to `_0`.
///
/// This allows LLVM to perform an optimization similar to the named return value optimization
