// Generated macro for InstNode (struct)
macro_rules! Depcrate_ir_layoutInstNode {
() => {
// Module: crate::ir::layout
// Provides: {"InstNode"}
// Dependencies: {}
# [derive (Clone , Debug , Default)] struct InstNode { # [doc = " The Block containing this instruction, or `None` if the instruction is not yet inserted."] block : PackedOption < Block > , prev : PackedOption < Inst > , next : PackedOption < Inst > , seq : SequenceNumber , }
};
}
