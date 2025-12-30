// Generated macro for encode_cl_type (function)
macro_rules! Depcrate_isa_riscv64_inst_encodeencode_cl_type {
() => {
// Module: crate::isa::riscv64::inst::encode
// Provides: {"encode_cl_type"}
// Dependencies: {}
pub fn encode_cl_type (op : ClOp , dest : WritableReg , base : Reg , imm : Uimm5) -> u16 { let size = match op { ClOp :: CFld | ClOp :: CLd => 8 , ClOp :: CLw => 4 , } ; encode_cs_cl_type_bits (op . op () , op . funct3 () , size , dest . to_reg () , base , imm) }
};
}
