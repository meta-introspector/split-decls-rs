// Generated macro for impl_2088 (impl)
macro_rules! Depcrate_isa_riscv64_inst_argsimpl_2088 {
() => {
// Module: crate::isa::riscv64::inst::args
// Provides: {"impl_2088"}
// Dependencies: {}
impl ZcbMemOp { pub fn funct6 (& self) -> u32 { match self { ZcbMemOp :: CLbu => 0b100_000 , ZcbMemOp :: CLhu | ZcbMemOp :: CLh => 0b100_001 , ZcbMemOp :: CSb => 0b100_010 , ZcbMemOp :: CSh => 0b100_011 , } } pub fn imm_bits (& self) -> u8 { match self { ZcbMemOp :: CLhu | ZcbMemOp :: CLh | ZcbMemOp :: CSh => 1 , ZcbMemOp :: CLbu | ZcbMemOp :: CSb => 2 , } } pub fn op (& self) -> COpcodeSpace { match self { ZcbMemOp :: CLbu | ZcbMemOp :: CLhu | ZcbMemOp :: CLh | ZcbMemOp :: CSb | ZcbMemOp :: CSh => { COpcodeSpace :: C0 } } } }
};
}
