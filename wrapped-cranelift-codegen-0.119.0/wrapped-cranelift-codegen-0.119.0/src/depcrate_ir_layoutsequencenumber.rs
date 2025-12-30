// Generated macro for SequenceNumber (type)
macro_rules! Depcrate_ir_layoutSequenceNumber {
() => {
// Module: crate::ir::layout
// Provides: {"SequenceNumber"}
// Dependencies: {}
# [doc = " Sequence numbers."] # [doc = ""] # [doc = " All instructions are given a sequence number that can be used to quickly determine"] # [doc = " their relative position in a block. The sequence numbers are not contiguous, but are assigned"] # [doc = " like line numbers in BASIC: 10, 20, 30, ..."] # [doc = ""] # [doc = " Sequence numbers are strictly increasing within a block, but are reset between blocks."] # [doc = ""] # [doc = " The result is that sequence numbers work like BASIC line numbers for the textual form of the IR."] type SequenceNumber = u32 ;
};
}
