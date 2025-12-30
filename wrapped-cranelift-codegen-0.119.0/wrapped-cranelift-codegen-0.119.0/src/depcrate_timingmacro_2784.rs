// Generated macro for macro_2784 (macro)
macro_rules! Depcrate_timingmacro_2784 {
() => {
// Module: crate::timing
// Provides: {"macro_2784"}
// Dependencies: {}
define_passes ! { process_file : "Processing test file" , parse_text : "Parsing textual Cranelift IR" , wasm_translate_module : "Translate WASM module" , wasm_translate_function : "Translate WASM function" , verifier : "Verify Cranelift IR" , compile : "Compilation passes" , try_incremental_cache : "Try loading from incremental cache" , store_incremental_cache : "Store in incremental cache" , flowgraph : "Control flow graph" , domtree : "Dominator tree" , loop_analysis : "Loop analysis" , preopt : "Pre-legalization rewriting" , egraph : "Egraph based optimizations" , gvn : "Global value numbering" , licm : "Loop invariant code motion" , unreachable_code : "Remove unreachable blocks" , remove_constant_phis : "Remove constant phi-nodes" , vcode_lower : "VCode lowering" , vcode_emit : "VCode emission" , vcode_emit_finish : "VCode emission finalization" , regalloc : "Register allocation" , regalloc_checker : "Register allocation symbolic verification" , layout_renumber : "Layout full renumbering" , canonicalize_nans : "Canonicalization of NaNs" , }
};
}
