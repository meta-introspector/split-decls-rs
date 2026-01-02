// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_mir_build/src/builder/expr/as_rvalue.rs
// Error: expected square brackets
// Problematic line: line 22

use crate::builder::expr::category::{Category, RvalueFunc};
use crate::builder::{BlockAnd, BlockAndExtension, Builder, NeedsTemporary};

impl<'a, 'tcx> Builder<'a, 'tcx> {
    /// Returns an rvalue suitable for use until the end of the current
    /// scope expression.
    ///
