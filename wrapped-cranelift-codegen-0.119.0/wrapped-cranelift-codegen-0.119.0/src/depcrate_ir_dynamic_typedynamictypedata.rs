// Generated macro for DynamicTypeData (struct)
macro_rules! Depcrate_ir_dynamic_typeDynamicTypeData {
() => {
// Module: crate::ir::dynamic_type
// Provides: {"DynamicTypeData"}
// Dependencies: {}
# [doc = " A dynamic type object which has a base vector type and a scaling factor."] # [derive (Clone , PartialEq , Hash)] # [cfg_attr (feature = "enable-serde" , derive (Serialize , Deserialize))] pub struct DynamicTypeData { # [doc = " Base vector type, this is the minimum size of the type."] pub base_vector_ty : Type , # [doc = " The dynamic scaling factor of the base vector type."] pub dynamic_scale : GlobalValue , }
};
}
