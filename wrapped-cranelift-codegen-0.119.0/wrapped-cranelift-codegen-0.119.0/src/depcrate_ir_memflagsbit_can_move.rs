// Generated macro for BIT_CAN_MOVE (const)
macro_rules! Depcrate_ir_memflagsBIT_CAN_MOVE {
() => {
// Module: crate::ir::memflags
// Provides: {"BIT_CAN_MOVE"}
// Dependencies: {}
# [doc = " Whether this memory operation may be freely moved by the optimizer so long"] # [doc = " as its data dependencies are satisfied. That is, by setting this flag, the"] # [doc = " producer is guaranteeing that this memory operation's safety is not guarded"] # [doc = " by outside-the-data-flow-graph properties, like implicit bounds-checking"] # [doc = " control dependencies."] const BIT_CAN_MOVE : u16 = 1 << 15 ;
};
}
