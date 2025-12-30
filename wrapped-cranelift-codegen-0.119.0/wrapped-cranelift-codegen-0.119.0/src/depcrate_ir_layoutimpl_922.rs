// Generated macro for impl_922 (impl)
macro_rules! Depcrate_ir_layoutimpl_922 {
() => {
// Module: crate::ir::layout
// Provides: {"impl_922"}
// Dependencies: {}
# [doc = " Use a layout reference in a for loop."] impl < 'f > IntoIterator for & 'f Layout { type Item = Block ; type IntoIter = Blocks < 'f > ; fn into_iter (self) -> Blocks < 'f > { self . blocks () } }
};
}
