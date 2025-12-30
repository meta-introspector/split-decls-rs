// Generated macro for impl_2083 (impl)
macro_rules! Depcrate_isa_riscv64_inst_argsimpl_2083 {
() => {
// Module: crate::isa::riscv64::inst::args
// Provides: {"impl_2083"}
// Dependencies: {}
impl CbOp { pub fn funct3 (& self) -> u32 { match self { CbOp :: CSrli | CbOp :: CSrai | CbOp :: CAndi => 0b100 , } } pub fn funct2 (& self) -> u32 { match self { CbOp :: CSrli => 0b00 , CbOp :: CSrai => 0b01 , CbOp :: CAndi => 0b10 , } } pub fn op (& self) -> COpcodeSpace { match self { CbOp :: CSrli | CbOp :: CSrai | CbOp :: CAndi => COpcodeSpace :: C1 , } } }
};
}
