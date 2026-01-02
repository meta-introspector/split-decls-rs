// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_codegen_cranelift/src/base.rs
// Error: expected square brackets
// Problematic line: line 22

use crate::pretty_clif::CommentWriter;
use crate::{codegen_f16_f128, enable_verifier};

pub(crate) struct CodegenedFunction {
    symbol_name: String,
    func_id: FuncId,
    func: Function,
