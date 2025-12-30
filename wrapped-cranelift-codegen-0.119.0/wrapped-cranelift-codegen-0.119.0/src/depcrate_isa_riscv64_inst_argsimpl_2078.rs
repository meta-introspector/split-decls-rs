// Generated macro for impl_2078 (impl)
macro_rules! Depcrate_isa_riscv64_inst_argsimpl_2078 {
() => {
// Module: crate::isa::riscv64::inst::args
// Provides: {"impl_2078"}
// Dependencies: {}
impl CrOp { pub fn funct4 (& self) -> u32 { match self { CrOp :: CMv | CrOp :: CJr => 0b1000 , CrOp :: CAdd | CrOp :: CJalr | CrOp :: CEbreak => 0b1001 , } } pub fn op (& self) -> COpcodeSpace { match self { CrOp :: CMv | CrOp :: CAdd | CrOp :: CJr | CrOp :: CJalr | CrOp :: CEbreak => COpcodeSpace :: C2 , } } }
};
}
