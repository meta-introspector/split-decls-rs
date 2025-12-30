// Generated macro for impl_1045 (impl)
macro_rules! Depcrate_ir_stackslotimpl_1045 {
() => {
// Module: crate::ir::stackslot
// Provides: {"impl_1045"}
// Dependencies: {}
impl DynamicStackSlotData { # [doc = " Create a stack slot with the specified byte size."] pub fn new (kind : StackSlotKind , dyn_ty : DynamicType) -> Self { assert ! (kind == StackSlotKind :: ExplicitDynamicSlot) ; Self { kind , dyn_ty } } # [doc = " Get the alignment in bytes of this stack slot given the stack pointer alignment."] pub fn alignment (& self , max_align : StackSize) -> StackSize { debug_assert ! (max_align . is_power_of_two ()) ; max_align } }
};
}
