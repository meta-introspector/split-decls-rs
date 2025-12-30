// Generated macro for impl_1040 (impl)
macro_rules! Depcrate_ir_stackslotimpl_1040 {
() => {
// Module: crate::ir::stackslot
// Provides: {"impl_1040"}
// Dependencies: {}
impl fmt :: Display for StackSlotKind { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { use self :: StackSlotKind :: * ; f . write_str (match * self { ExplicitSlot => "explicit_slot" , ExplicitDynamicSlot => "explicit_dynamic_slot" , }) } }
};
}
