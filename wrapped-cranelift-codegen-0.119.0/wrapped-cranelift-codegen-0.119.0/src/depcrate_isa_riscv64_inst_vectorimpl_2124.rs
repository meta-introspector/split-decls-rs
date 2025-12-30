// Generated macro for impl_2124 (impl)
macro_rules! Depcrate_isa_riscv64_inst_vectorimpl_2124 {
() => {
// Module: crate::isa::riscv64::inst::vector
// Provides: {"impl_2124"}
// Dependencies: {}
impl VecTailMode { pub fn encode (& self) -> u32 { match self { VecTailMode :: Agnostic => 1 , VecTailMode :: Undisturbed => 0 , } } }
};
}
