// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_codegen_ssa/src/assert_module_sources.rs
// Error: expected square brackets
// Problematic line: line 42


use crate::errors;

#[allow(missing_docs)]
pub fn assert_module_sources(tcx: TyCtxt<'_>, set_reuse: &dyn Fn(&mut CguReuseTracker)) {
    tcx.dep_graph.with_ignore(|| {
        if tcx.sess.opts.incremental.is_none() {
