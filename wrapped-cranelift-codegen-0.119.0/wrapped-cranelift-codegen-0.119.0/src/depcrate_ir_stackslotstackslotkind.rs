// Generated macro for StackSlotKind (enum)
macro_rules! Depcrate_ir_stackslotStackSlotKind {
() => {
// Module: crate::ir::stackslot
// Provides: {"StackSlotKind"}
// Dependencies: {}
# [doc = " The kind of a stack slot."] # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash)] # [cfg_attr (feature = "enable-serde" , derive (Serialize , Deserialize))] pub enum StackSlotKind { # [doc = " An explicit stack slot. This is a chunk of stack memory for use by the `stack_load`"] # [doc = " and `stack_store` instructions."] ExplicitSlot , # [doc = " An explicit stack slot for dynamic vector types. This is a chunk of stack memory"] # [doc = " for use by the `dynamic_stack_load` and `dynamic_stack_store` instructions."] ExplicitDynamicSlot , }
};
}
