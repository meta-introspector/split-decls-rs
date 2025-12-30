// Generated macro for impl_2086 (impl)
macro_rules! Depcrate_isa_riscv64_inst_argsimpl_2086 {
() => {
// Module: crate::isa::riscv64::inst::args
// Provides: {"impl_2086"}
// Dependencies: {}
impl ClOp { pub fn funct3 (& self) -> u32 { match self { ClOp :: CFld => 0b001 , ClOp :: CLw => 0b010 , ClOp :: CLd => 0b011 , } } pub fn op (& self) -> COpcodeSpace { match self { ClOp :: CLw | ClOp :: CLd | ClOp :: CFld => COpcodeSpace :: C0 , } } }
};
}
