// Generated macro for impl_351 (impl)
macro_rules! Depcrate_merge_operatorimpl_351 {
() => {
// Module: crate::merge_operator
// Provides: {"impl_351"}
// Dependencies: {}
impl < 'a > Iterator for MergeOperandsIter < 'a > { type Item = & 'a [u8] ; fn next (& mut self) -> Option < Self :: Item > { let operand = self . operands . get_operand (self . cursor) ? ; self . cursor += 1 ; Some (operand) } fn size_hint (& self) -> (usize , Option < usize >) { let remaining = self . operands . num_operands - self . cursor ; (remaining , Some (remaining)) } }
};
}
