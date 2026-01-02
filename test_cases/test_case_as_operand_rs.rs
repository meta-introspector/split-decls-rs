// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_mir_build/src/builder/expr/as_operand.rs
// Error: expected square brackets
// Problematic line: line 10

use crate::builder::expr::category::Category;
use crate::builder::{BlockAnd, BlockAndExtension, Builder, NeedsTemporary};

impl<'a, 'tcx> Builder<'a, 'tcx> {
    /// Construct a temporary lifetime restricted to just the local scope
    pub(crate) fn local_temp_lifetime(&self) -> TempLifetime {
        let local_scope = self.local_scope();
