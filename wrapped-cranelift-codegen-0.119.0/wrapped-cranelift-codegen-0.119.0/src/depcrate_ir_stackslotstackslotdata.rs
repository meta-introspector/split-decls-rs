// Generated macro for StackSlotData (struct)
macro_rules! Depcrate_ir_stackslotStackSlotData {
() => {
// Module: crate::ir::stackslot
// Provides: {"StackSlotData"}
// Dependencies: {}
# [doc = " Contents of a stack slot."] # [derive (Clone , Debug , PartialEq , Eq , Hash)] # [cfg_attr (feature = "enable-serde" , derive (Serialize , Deserialize))] pub struct StackSlotData { # [doc = " The kind of stack slot."] pub kind : StackSlotKind , # [doc = " Size of stack slot in bytes."] pub size : StackSize , # [doc = " Alignment of stack slot as a power-of-two exponent (log2"] # [doc = " value). The stack slot will be at least this aligned; it may"] # [doc = " be aligned according to other considerations, such as minimum"] # [doc = " stack slot size or machine word size, as well."] pub align_shift : u8 , }
};
}
