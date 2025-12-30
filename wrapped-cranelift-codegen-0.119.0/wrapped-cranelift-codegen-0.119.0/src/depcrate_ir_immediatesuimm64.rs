// Generated macro for Uimm64 (struct)
macro_rules! Depcrate_ir_immediatesUimm64 {
() => {
// Module: crate::ir::immediates
// Provides: {"Uimm64"}
// Dependencies: {}
# [doc = " 64-bit immediate unsigned integer operand."] # [doc = ""] # [doc = " A `Uimm64` operand can also be used to represent immediate values of smaller integer types by"] # [doc = " zero-extending to `i64`."] # [derive (Copy , Clone , PartialEq , Eq , Debug , Hash)] # [cfg_attr (feature = "enable-serde" , derive (Serialize , Deserialize))] pub struct Uimm64 (u64) ;
};
}
