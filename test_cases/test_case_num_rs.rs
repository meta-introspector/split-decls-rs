// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_codegen_cranelift/src/num.rs
// Error: expected square brackets
// Problematic line: line 6

use crate::codegen_f16_f128;
use crate::prelude::*;

fn bin_op_to_intcc(bin_op: BinOp, signed: bool) -> IntCC {
    use BinOp::*;
    use IntCC::*;
    match bin_op {
