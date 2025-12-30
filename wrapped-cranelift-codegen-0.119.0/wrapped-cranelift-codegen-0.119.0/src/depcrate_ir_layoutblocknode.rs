// Generated macro for BlockNode (struct)
macro_rules! Depcrate_ir_layoutBlockNode {
() => {
// Module: crate::ir::layout
// Provides: {"BlockNode"}
// Dependencies: {}
# [doc = " A single node in the linked-list of blocks."] # [derive (Clone , Debug , Default , PartialEq , Hash)] struct BlockNode { prev : PackedOption < Block > , next : PackedOption < Block > , first_inst : PackedOption < Inst > , last_inst : PackedOption < Inst > , cold : bool , }
};
}
