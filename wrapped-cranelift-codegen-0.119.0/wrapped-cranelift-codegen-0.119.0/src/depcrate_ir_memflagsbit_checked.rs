// Generated macro for BIT_CHECKED (const)
macro_rules! Depcrate_ir_memflagsBIT_CHECKED {
() => {
// Module: crate::ir::memflags
// Provides: {"BIT_CHECKED"}
// Dependencies: {}
# [doc = " Check this load or store for safety when using the"] # [doc = " proof-carrying-code framework. The address must have a"] # [doc = " `PointsTo` fact attached with a sufficiently large valid range"] # [doc = " for the accessed size."] const BIT_CHECKED : u16 = 1 << 4 ;
};
}
