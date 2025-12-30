// Generated macro for ModuleConfig (struct)
macro_rules! Depcrate_back_writeModuleConfig {
() => {
// Module: crate::back::write
// Provides: {"ModuleConfig"}
// Dependencies: {}
# [doc = " Module-specific configuration for `optimize_and_codegen`."] pub struct ModuleConfig { # [doc = " Names of additional optimization passes to run."] pub passes : Vec < String > , # [doc = " Some(level) to optimize at a certain level, or None to run"] # [doc = " absolutely no optimizations (used for the allocator module)."] pub opt_level : Option < config :: OptLevel > , pub pgo_gen : SwitchWithOptPath , pub pgo_use : Option < PathBuf > , pub pgo_sample_use : Option < PathBuf > , pub debug_info_for_profiling : bool , pub instrument_coverage : bool , pub sanitizer : SanitizerSet , pub sanitizer_recover : SanitizerSet , pub sanitizer_dataflow_abilist : Vec < String > , pub sanitizer_memory_track_origins : usize , pub emit_pre_lto_bc : bool , pub emit_no_opt_bc : bool , pub emit_bc : bool , pub emit_ir : bool , pub emit_asm : bool , pub emit_obj : EmitObj , pub emit_thin_lto : bool , pub emit_thin_lto_summary : bool , pub verify_llvm_ir : bool , pub lint_llvm_ir : bool , pub no_prepopulate_passes : bool , pub no_builtins : bool , pub vectorize_loop : bool , pub vectorize_slp : bool , pub merge_functions : bool , pub emit_lifetime_markers : bool , pub llvm_plugins : Vec < String > , pub autodiff : Vec < config :: AutoDiff > , pub offload : Vec < config :: Offload > , }
};
}
