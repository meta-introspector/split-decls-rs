// Generated macro for impl_2087 (impl)
macro_rules! Depcrate_isa_riscv64_inst_argsimpl_2087 {
() => {
// Module: crate::isa::riscv64::inst::args
// Provides: {"impl_2087"}
// Dependencies: {}
impl CsznOp { pub fn funct6 (& self) -> u32 { match self { CsznOp :: CNot | CsznOp :: CZextw | CsznOp :: CZextb | CsznOp :: CZexth | CsznOp :: CSextb | CsznOp :: CSexth => 0b100_111 , } } pub fn funct5 (& self) -> u32 { match self { CsznOp :: CNot => 0b11_101 , CsznOp :: CZextb => 0b11_000 , CsznOp :: CZexth => 0b11_010 , CsznOp :: CZextw => 0b11_100 , CsznOp :: CSextb => 0b11_001 , CsznOp :: CSexth => 0b11_011 , } } pub fn op (& self) -> COpcodeSpace { match self { CsznOp :: CNot | CsznOp :: CZextb | CsznOp :: CZexth | CsznOp :: CZextw | CsznOp :: CSextb | CsznOp :: CSexth => COpcodeSpace :: C1 , } } }
};
}
