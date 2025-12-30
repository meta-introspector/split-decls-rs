// Generated macro for MemoryTypeField (struct)
macro_rules! Depcrate_ir_memtypeMemoryTypeField {
() => {
// Module: crate::ir::memtype
// Provides: {"MemoryTypeField"}
// Dependencies: {}
# [doc = " One field in a memory type."] # [derive (Clone , PartialEq , Hash)] # [cfg_attr (feature = "enable-serde" , derive (Serialize , Deserialize))] pub struct MemoryTypeField { # [doc = " The offset of this field in the memory type."] pub offset : u64 , # [doc = " The primitive type of the value in this field. Accesses to the"] # [doc = " field must use this type (i.e., cannot bitcast/type-pun in"] # [doc = " memory)."] pub ty : Type , # [doc = " A proof-carrying-code fact about this value, if any."] pub fact : Option < Fact > , # [doc = " Whether this field is read-only, i.e., stores should be"] # [doc = " disallowed."] pub readonly : bool , }
};
}
