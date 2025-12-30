// Generated macro for OptimizationPassInfo (struct)
macro_rules! Depcrate_llvm_ir_extractorOptimizationPassInfo {
() => {
// Module: crate::llvm_ir_extractor
// Provides: {"OptimizationPassInfo"}
// Dependencies: {}
# [doc = " Detailed optimization pass information"] # [derive (Debug , Clone , Serialize , Deserialize)] pub struct OptimizationPassInfo { # [doc = " Name of the optimization pass"] pub pass_name : String , # [doc = " Pass type (function, module, loop, etc.)"] pub pass_type : String , # [doc = " IR instructions before pass"] pub instructions_before : u32 , # [doc = " IR instructions after pass"] pub instructions_after : u32 , # [doc = " Estimated performance impact"] pub performance_impact : f32 , # [doc = " Pass execution time"] pub execution_time_ms : u64 , }
};
}
