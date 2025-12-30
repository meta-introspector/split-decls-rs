// Generated macro for impl_2085 (impl)
macro_rules! Depcrate_isa_riscv64_inst_argsimpl_2085 {
() => {
// Module: crate::isa::riscv64::inst::args
// Provides: {"impl_2085"}
// Dependencies: {}
impl CsOp { pub fn funct3 (& self) -> u32 { match self { CsOp :: CFsd => 0b101 , CsOp :: CSw => 0b110 , CsOp :: CSd => 0b111 , } } pub fn op (& self) -> COpcodeSpace { match self { CsOp :: CSw | CsOp :: CSd | CsOp :: CFsd => COpcodeSpace :: C0 , } } }
};
}
