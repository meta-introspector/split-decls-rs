// Generated macro for impl_2047 (impl)
macro_rules! Depcrate_isa_riscv64_inst_argsimpl_2047 {
() => {
// Module: crate::isa::riscv64::inst::args
// Provides: {"impl_2047"}
// Dependencies: {}
impl BranchFunct3 { pub (crate) fn funct3 (self) -> u32 { match self { BranchFunct3 :: Eq => 0b000 , BranchFunct3 :: Ne => 0b001 , BranchFunct3 :: Lt => 0b100 , BranchFunct3 :: Ge => 0b101 , BranchFunct3 :: Ltu => 0b110 , BranchFunct3 :: Geu => 0b111 , } } }
};
}
