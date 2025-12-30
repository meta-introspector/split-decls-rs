// Generated macro for encode_cs_type (function)
macro_rules! Depcrate_isa_riscv64_inst_encodeencode_cs_type {
() => {
// Module: crate::isa::riscv64::inst::encode
// Provides: {"encode_cs_type"}
// Dependencies: {}
pub fn encode_cs_type (op : CsOp , src : Reg , base : Reg , imm : Uimm5) -> u16 { let size = match op { CsOp :: CFsd | CsOp :: CSd => 8 , CsOp :: CSw => 4 , } ; encode_cs_cl_type_bits (op . op () , op . funct3 () , size , src , base , imm) }
};
}
