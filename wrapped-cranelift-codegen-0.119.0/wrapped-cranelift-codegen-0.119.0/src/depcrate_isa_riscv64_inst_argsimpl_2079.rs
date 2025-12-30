// Generated macro for impl_2079 (impl)
macro_rules! Depcrate_isa_riscv64_inst_argsimpl_2079 {
() => {
// Module: crate::isa::riscv64::inst::args
// Provides: {"impl_2079"}
// Dependencies: {}
impl CaOp { pub fn funct2 (& self) -> u32 { match self { CaOp :: CAnd => 0b11 , CaOp :: COr => 0b10 , CaOp :: CXor => 0b01 , CaOp :: CSub => 0b00 , CaOp :: CAddw => 0b01 , CaOp :: CSubw => 0b00 , CaOp :: CMul => 0b10 , } } pub fn funct6 (& self) -> u32 { match self { CaOp :: CAnd | CaOp :: COr | CaOp :: CXor | CaOp :: CSub => 0b100_011 , CaOp :: CSubw | CaOp :: CAddw | CaOp :: CMul => 0b100_111 , } } pub fn op (& self) -> COpcodeSpace { match self { CaOp :: CAnd | CaOp :: COr | CaOp :: CXor | CaOp :: CSub | CaOp :: CAddw | CaOp :: CSubw | CaOp :: CMul => COpcodeSpace :: C1 , } } }
};
}
