// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_mir_transform/src/prettify.rs
// Error: expected square brackets
// Problematic line: line 14

use rustc_middle::ty::TyCtxt;
use rustc_session::Session;

/// Rearranges the basic blocks into a *reverse post-order*.
///
/// Thus after this pass, all the successors of a block are later than it in the
/// `IndexVec`, unless that successor is a back-edge (such as from a loop).
