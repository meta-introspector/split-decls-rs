// Generated macro for impl_1042 (impl)
macro_rules! Depcrate_ir_stackslotimpl_1042 {
() => {
// Module: crate::ir::stackslot
// Provides: {"impl_1042"}
// Dependencies: {}
impl StackSlotData { # [doc = " Create a stack slot with the specified byte size and alignment."] pub fn new (kind : StackSlotKind , size : StackSize , align_shift : u8) -> Self { Self { kind , size , align_shift , } } }
};
}
