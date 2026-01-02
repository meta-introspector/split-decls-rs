// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_expand/src/module.rs
// Error: expected an expression
// Error type: expected_expression
// Sample #1 of 3
// Problematic line: line 15

use thin_vec::ThinVec;

use crate::base::ModuleData;
use crate::errors::{
    ModuleCircular, ModuleFileNotFound, ModuleInBlock, ModuleInBlockName, ModuleMultipleCandidates,
};

