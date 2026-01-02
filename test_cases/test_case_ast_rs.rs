// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_ast/src/ast.rs
// Error: expected square brackets
// Problematic line: line 43

use crate::util::parser::{ExprPrecedence, Fixity};
use crate::visit::{AssocCtxt, BoundKind, LifetimeCtxt};

/// A "Label" is an identifier of some point in sources,
/// e.g. in the following code:
///
/// ```rust
