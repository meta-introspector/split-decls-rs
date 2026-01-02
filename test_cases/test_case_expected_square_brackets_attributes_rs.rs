// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_codegen_llvm/src/attributes.rs
// Error: expected square brackets
// Error type: expected_square_brackets
// Sample #1 of 3
// Problematic line: line 19

use crate::value::Value;
use crate::{attributes, llvm_util};

pub(crate) fn apply_to_llfn(llfn: &Value, idx: AttributePlace, attrs: &[&Attribute]) {
    if !attrs.is_empty() {
        llvm::AddFunctionAttributes(llfn, idx, attrs);
    }
