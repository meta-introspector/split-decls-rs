// Generated macro for DynamicStackSlotData (struct)
macro_rules! Depcrate_ir_stackslotDynamicStackSlotData {
() => {
// Module: crate::ir::stackslot
// Provides: {"DynamicStackSlotData"}
// Dependencies: {}
# [doc = " Contents of a dynamic stack slot."] # [derive (Clone , Debug , PartialEq , Eq , Hash)] # [cfg_attr (feature = "enable-serde" , derive (Serialize , Deserialize))] pub struct DynamicStackSlotData { # [doc = " The kind of stack slot."] pub kind : StackSlotKind , # [doc = " The type of this slot."] pub dyn_ty : DynamicType , }
};
}
