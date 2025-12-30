// Generated macro for impl_2071 (impl)
macro_rules! Depcrate_isa_riscv64_inst_argsimpl_2071 {
() => {
// Module: crate::isa::riscv64::inst::args
// Provides: {"impl_2071"}
// Dependencies: {}
impl CsrRegOP { pub (crate) fn funct3 (self) -> u32 { match self { CsrRegOP :: CsrRW => 0b001 , CsrRegOP :: CsrRS => 0b010 , CsrRegOP :: CsrRC => 0b011 , } } pub (crate) fn opcode (self) -> u32 { 0b1110011 } pub (crate) fn name (self) -> & 'static str { match self { CsrRegOP :: CsrRW => "csrrw" , CsrRegOP :: CsrRS => "csrrs" , CsrRegOP :: CsrRC => "csrrc" , } } }
};
}
