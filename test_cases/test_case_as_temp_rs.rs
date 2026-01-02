// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_mir_build/src/builder/expr/as_temp.rs
// Error: expected square brackets
// Problematic line: line 13

use crate::builder::scope::DropKind;
use crate::builder::{BlockAnd, BlockAndExtension, Builder};

impl<'a, 'tcx> Builder<'a, 'tcx> {
    /// Compile `expr` into a fresh temporary. This is used when building
    /// up rvalues so as to freeze the value that will be consumed.
    pub(crate) fn as_temp(
