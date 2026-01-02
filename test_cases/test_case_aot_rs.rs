// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_codegen_cranelift/src/driver/aot.rs
// Error: expected square brackets
// Problematic line: line 15

use rustc_codegen_ssa::assert_module_sources::CguReuse;
use rustc_codegen_ssa::back::link::ensure_removed;
use rustc_codegen_ssa::base::determine_cgu_reuse;
use rustc_codegen_ssa::{
    CodegenResults, CompiledModule, CrateInfo, ModuleKind, errors as ssa_errors,
};
use rustc_data_structures::profiling::SelfProfilerRef;
