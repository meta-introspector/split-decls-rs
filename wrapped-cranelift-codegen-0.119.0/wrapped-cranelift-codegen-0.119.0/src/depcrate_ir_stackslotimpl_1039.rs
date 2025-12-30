// Generated macro for impl_1039 (impl)
macro_rules! Depcrate_ir_stackslotimpl_1039 {
() => {
// Module: crate::ir::stackslot
// Provides: {"impl_1039"}
// Dependencies: {}
impl FromStr for StackSlotKind { type Err = () ; fn from_str (s : & str) -> Result < Self , () > { use self :: StackSlotKind :: * ; match s { "explicit_slot" => Ok (ExplicitSlot) , "explicit_dynamic_slot" => Ok (ExplicitDynamicSlot) , _ => Err (()) , } } }
};
}
