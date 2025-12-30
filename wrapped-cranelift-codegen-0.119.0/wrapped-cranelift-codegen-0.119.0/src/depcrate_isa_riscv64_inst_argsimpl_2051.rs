// Generated macro for impl_2051 (impl)
macro_rules! Depcrate_isa_riscv64_inst_argsimpl_2051 {
() => {
// Module: crate::isa::riscv64::inst::args
// Provides: {"impl_2051"}
// Dependencies: {}
impl FpuOPRRRR { pub (crate) fn op_name (self , width : FpuOPWidth) -> String { match self { Self :: Fmadd => format ! ("fmadd.{width}") , Self :: Fmsub => format ! ("fmsub.{width}") , Self :: Fnmsub => format ! ("fnmsub.{width}") , Self :: Fnmadd => format ! ("fnmadd.{width}") , } } pub (crate) fn opcode (self) -> u32 { match self { Self :: Fmadd => 0b1000011 , Self :: Fmsub => 0b1000111 , Self :: Fnmsub => 0b1001011 , Self :: Fnmadd => 0b1001111 , } } }
};
}
