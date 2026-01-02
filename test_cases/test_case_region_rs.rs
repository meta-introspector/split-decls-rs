// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_middle/src/middle/region.rs
// Error: expected square brackets
// Problematic line: line 22

use crate::mir::BackwardIncompatibleDropReason;
use crate::ty::TyCtxt;

/// Represents a statically-describable scope that can be used to
/// bound the lifetime/region for values.
///
/// `Node(node_id)`: Any AST node that has any scope at all has the
