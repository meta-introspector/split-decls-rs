// Generated macro for impl_2084 (impl)
macro_rules! Depcrate_isa_riscv64_inst_argsimpl_2084 {
() => {
// Module: crate::isa::riscv64::inst::args
// Provides: {"impl_2084"}
// Dependencies: {}
impl CssOp { pub fn funct3 (& self) -> u32 { match self { CssOp :: CFsdsp => 0b101 , CssOp :: CSwsp => 0b110 , CssOp :: CSdsp => 0b111 , } } pub fn op (& self) -> COpcodeSpace { match self { CssOp :: CSwsp | CssOp :: CSdsp | CssOp :: CFsdsp => COpcodeSpace :: C2 , } } }
};
}
