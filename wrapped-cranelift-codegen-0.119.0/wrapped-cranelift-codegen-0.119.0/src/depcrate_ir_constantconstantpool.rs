// Generated macro for ConstantPool (struct)
macro_rules! Depcrate_ir_constantConstantPool {
() => {
// Module: crate::ir::constant
// Provides: {"ConstantPool"}
// Dependencies: {}
# [doc = " Maintains the mapping between a constant handle (i.e.  [`Constant`]) and"] # [doc = " its constant data (i.e.  [`ConstantData`])."] # [derive (Clone , PartialEq , Hash)] # [cfg_attr (feature = "enable-serde" , derive (Serialize , Deserialize))] pub struct ConstantPool { # [doc = " This mapping maintains the insertion order as long as Constants are created with"] # [doc = " sequentially increasing integers."] # [doc = ""] # [doc = " It is important that, by construction, no entry in that list gets removed. If that ever"] # [doc = " need to happen, don't forget to update the `Constant` generation scheme."] handles_to_values : BTreeMap < Constant , ConstantData > , # [doc = " Mapping of hashed `ConstantData` to the index into the other hashmap."] # [doc = ""] # [doc = " This allows for deduplication of entries into the `handles_to_values` mapping."] values_to_handles : BTreeMap < ConstantData , Constant > , }
};
}
