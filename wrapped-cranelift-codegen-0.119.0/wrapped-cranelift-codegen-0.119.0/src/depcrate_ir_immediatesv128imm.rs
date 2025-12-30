// Generated macro for V128Imm (struct)
macro_rules! Depcrate_ir_immediatesV128Imm {
() => {
// Module: crate::ir::immediates
// Provides: {"V128Imm"}
// Dependencies: {}
# [doc = " A 128-bit immediate operand."] # [doc = ""] # [doc = " This is used as an immediate value in SIMD instructions."] # [derive (Copy , Clone , PartialEq , Eq , Debug , Hash)] # [cfg_attr (feature = "enable-serde" , derive (Serialize , Deserialize))] pub struct V128Imm (pub [u8 ; 16]) ;
};
}
