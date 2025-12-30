// Generated macro for Constant (struct)
macro_rules! Depcrate_ir_entitiesConstant {
() => {
// Module: crate::ir::entities
// Provides: {"Constant"}
// Dependencies: {}
# [doc = " An opaque reference to a constant."] # [doc = ""] # [doc = " You can store [`ConstantData`](super::ConstantData) in a"] # [doc = " [`ConstantPool`](super::ConstantPool) for efficient storage and retrieval."] # [doc = " See [`ConstantPool::insert`](super::ConstantPool::insert)."] # [doc = ""] # [doc = " While the order is stable, it is arbitrary and does not necessarily resemble the order in which"] # [doc = " the constants are written in the constant pool."] # [derive (Copy , Clone , PartialEq , Eq , Hash , Ord , PartialOrd)] # [cfg_attr (feature = "enable-serde" , derive (Serialize , Deserialize))] pub struct Constant (u32) ;
};
}
