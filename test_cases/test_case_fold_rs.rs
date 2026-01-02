// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_type_ir/src/fold.rs
// Error: expected square brackets
// Problematic line: line 60

use crate::visit::{TypeVisitable, TypeVisitableExt as _};
use crate::{self as ty, Interner, TypeFlags};

/// This trait is implemented for every type that can be folded,
/// providing the skeleton of the traversal.
///
/// To implement this conveniently, use the derive macro located in
