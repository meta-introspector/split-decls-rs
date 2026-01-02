// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_const_eval/src/interpret/intern.rs
// Error: expected square brackets
// Problematic line: line 22

use rustc_hir as hir;
use rustc_hir::definitions::{DefPathData, DisambiguatorState};
use rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs;
use rustc_middle::mir::interpret::{
    AllocBytes, ConstAllocation, CtfeProvenance, InterpResult, Provenance,
};
use rustc_middle::query::TyCtxtAt;
