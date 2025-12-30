// Generated macro for LOCAL_LIMIT (const)
macro_rules! Depcrate_ir_layoutLOCAL_LIMIT {
() => {
// Module: crate::ir::layout
// Provides: {"LOCAL_LIMIT"}
// Dependencies: {}
# [doc = " Limit on the sequence number range we'll renumber locally. If this limit is exceeded, we'll"] # [doc = " switch to a full block renumbering."] const LOCAL_LIMIT : SequenceNumber = 100 * MINOR_STRIDE ;
};
}
