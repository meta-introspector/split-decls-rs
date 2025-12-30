// Generated macro for FunctionAlignment (struct)
macro_rules! Depcrate_isaFunctionAlignment {
() => {
// Module: crate::isa
// Provides: {"FunctionAlignment"}
// Dependencies: {}
# [doc = " Function alignment specifications as required by an ISA, returned by"] # [doc = " [`TargetIsa::function_alignment`]."] # [derive (Copy , Clone)] pub struct FunctionAlignment { # [doc = " The minimum alignment required by an ISA, where all functions must be"] # [doc = " aligned to at least this amount."] pub minimum : u32 , # [doc = " A \"preferred\" alignment which should be used for more"] # [doc = " performance-sensitive situations. This can involve cache-line-aligning"] # [doc = " for example to get more of a small function into fewer cache lines."] pub preferred : u32 , }
};
}
