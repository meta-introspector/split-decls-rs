// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_mir_transform/src/deduce_param_attrs.rs
// Error: expected square brackets
// Problematic line: line 15

use rustc_middle::ty::{self, DeducedParamAttrs, Ty, TyCtxt};
use rustc_session::config::OptLevel;

/// A visitor that determines which arguments have been mutated. We can't use the mutability field
/// on LocalDecl for this because it has no meaning post-optimization.
struct DeduceReadOnly {
    /// Each bit is indexed by argument number, starting at zero (so 0 corresponds to local decl
