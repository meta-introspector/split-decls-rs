// Generated macro for impl_2073 (impl)
macro_rules! Depcrate_isa_riscv64_inst_argsimpl_2073 {
() => {
// Module: crate::isa::riscv64::inst::args
// Provides: {"impl_2073"}
// Dependencies: {}
impl CsrImmOP { pub (crate) fn funct3 (self) -> u32 { match self { CsrImmOP :: CsrRWI => 0b101 , CsrImmOP :: CsrRSI => 0b110 , CsrImmOP :: CsrRCI => 0b111 , } } pub (crate) fn opcode (self) -> u32 { 0b1110011 } pub (crate) fn name (self) -> & 'static str { match self { CsrImmOP :: CsrRWI => "csrrwi" , CsrImmOP :: CsrRSI => "csrrsi" , CsrImmOP :: CsrRCI => "csrrci" , } } }
};
}
