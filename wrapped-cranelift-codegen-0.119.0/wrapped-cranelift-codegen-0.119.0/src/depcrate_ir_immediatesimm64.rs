// Generated macro for Imm64 (struct)
macro_rules! Depcrate_ir_immediatesImm64 {
() => {
// Module: crate::ir::immediates
// Provides: {"Imm64"}
// Dependencies: {}
# [doc = " 64-bit immediate signed integer operand."] # [doc = ""] # [doc = " An `Imm64` operand can also be used to represent immediate values of smaller integer types by"] # [doc = " sign-extending to `i64`."] # [derive (Copy , Clone , PartialEq , Eq , Debug , Hash)] # [cfg_attr (feature = "enable-serde" , derive (Serialize , Deserialize))] pub struct Imm64 (i64) ;
};
}
