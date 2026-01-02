// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_mir_build/src/builder/matches/user_ty.rs
// Error: expected square brackets
// Problematic line: line 15

use rustc_middle::ty::{AdtDef, UserTypeAnnotationIndex};
use rustc_span::Symbol;

/// One of a list of "operations" that can be used to lazily build projections
/// of user-specified types.
#[derive(Clone, Debug)]
pub(crate) enum ProjectedUserTypesOp {
