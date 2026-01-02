// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_borrowck/src/diagnostics/region_name.rs
// Error: expected square brackets
// Problematic line: line 21

use crate::MirBorrowckCtxt;
use crate::universal_regions::DefiningTy;

/// A name for a particular region used in emitting diagnostics. This name could be a generated
/// name like `'1`, a name used by the user like `'a`, or a name like `'static`.
#[derive(Debug, Clone, Copy)]
pub(crate) struct RegionName {
