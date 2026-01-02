// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_codegen_ssa/src/size_of_val.rs
// Error: expected square brackets
// Problematic line: line 15

use crate::traits::*;
use crate::{common, meth};

pub fn size_and_align_of_dst<'a, 'tcx, Bx: BuilderMethods<'a, 'tcx>>(
    bx: &mut Bx,
    t: Ty<'tcx>,
    info: Option<Bx::Value>,
