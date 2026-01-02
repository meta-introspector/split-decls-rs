// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_ast/src/expand/autodiff_attrs.rs
// Error: expected square brackets
// Problematic line: line 12

use crate::expand::{Decodable, Encodable, HashStable_Generic};
use crate::{Ty, TyKind};

/// Forward and Reverse Mode are well known names for automatic differentiation implementations.
/// Enzyme does support both, but with different semantics, see DiffActivity. The First variants
/// are a hack to support higher order derivatives. We need to compute first order derivatives
/// before we compute second order derivatives, otherwise we would differentiate our placeholder
