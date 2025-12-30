// Generated macro for impl_921 (impl)
macro_rules! Depcrate_ir_layoutimpl_921 {
() => {
// Module: crate::ir::layout
// Provides: {"impl_921"}
// Dependencies: {}
impl < 'f > Iterator for Blocks < 'f > { type Item = Block ; fn next (& mut self) -> Option < Block > { match self . next { Some (block) => { self . next = self . layout . next_block (block) ; Some (block) } None => None , } } }
};
}
