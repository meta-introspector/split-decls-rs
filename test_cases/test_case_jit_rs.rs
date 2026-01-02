// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_codegen_cranelift/src/driver/jit.rs
// Error: expected square brackets
// Problematic line: line 19

use crate::prelude::*;
use crate::unwind_module::UnwindModule;

fn create_jit_module(tcx: TyCtxt<'_>) -> (UnwindModule<JITModule>, CodegenCx) {
    let crate_info = CrateInfo::new(tcx, "dummy_target_cpu".to_string());

    let isa = crate::build_isa(tcx.sess, true);
