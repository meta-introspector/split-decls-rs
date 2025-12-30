// Generated macro for impl_604 (impl)
macro_rules! Depcrate_ir_dfgimpl_604 {
() => {
// Module: crate::ir::dfg
// Provides: {"impl_604"}
// Dependencies: {}
impl < 'a > Iterator for Values < 'a > { type Item = Value ; fn next (& mut self) -> Option < Self :: Item > { self . inner . by_ref () . find (| kv | valid_valuedata (* kv . 1)) . map (| kv | kv . 0) } }
};
}
