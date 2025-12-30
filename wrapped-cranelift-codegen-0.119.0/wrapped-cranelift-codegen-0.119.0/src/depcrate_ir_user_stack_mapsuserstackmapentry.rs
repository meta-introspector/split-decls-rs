// Generated macro for UserStackMapEntry (struct)
macro_rules! Depcrate_ir_user_stack_mapsUserStackMapEntry {
() => {
// Module: crate::ir::user_stack_maps
// Provides: {"UserStackMapEntry"}
// Dependencies: {}
# [doc = " A stack map entry describes a single GC-managed value and its location on"] # [doc = " the stack."] # [doc = ""] # [doc = " A stack map entry is associated with a particular instruction, and that"] # [doc = " instruction must be a safepoint. The GC-managed value must be stored in the"] # [doc = " described location across this entry's instruction."] # [derive (Clone , Debug , PartialEq , Hash)] # [cfg_attr (feature = "enable-serde" , derive (serde_derive :: Serialize , serde_derive :: Deserialize))] pub struct UserStackMapEntry { # [doc = " The type of the value stored in this stack map entry."] pub ty : ir :: Type , # [doc = " The stack slot that this stack map entry is within."] pub slot : ir :: StackSlot , # [doc = " The offset within the stack slot where this entry's value can be found."] pub offset : u32 , }
};
}
