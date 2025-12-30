// Generated macro for impl_2081 (impl)
macro_rules! Depcrate_isa_riscv64_inst_argsimpl_2081 {
() => {
// Module: crate::isa::riscv64::inst::args
// Provides: {"impl_2081"}
// Dependencies: {}
impl CiOp { pub fn funct3 (& self) -> u32 { match self { CiOp :: CAddi | CiOp :: CSlli => 0b000 , CiOp :: CAddiw | CiOp :: CFldsp => 0b001 , CiOp :: CLi | CiOp :: CLwsp => 0b010 , CiOp :: CAddi16sp | CiOp :: CLui | CiOp :: CLdsp => 0b011 , } } pub fn op (& self) -> COpcodeSpace { match self { CiOp :: CAddi | CiOp :: CAddiw | CiOp :: CAddi16sp | CiOp :: CLi | CiOp :: CLui => { COpcodeSpace :: C1 } CiOp :: CSlli | CiOp :: CLwsp | CiOp :: CLdsp | CiOp :: CFldsp => COpcodeSpace :: C2 , } } }
};
}
