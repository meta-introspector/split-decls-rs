// Generated macro for impl_2136 (impl)
macro_rules! Depcrate_isa_riscv64_inst_vectorimpl_2136 {
() => {
// Module: crate::isa::riscv64::inst::vector
// Provides: {"impl_2136"}
// Dependencies: {}
impl VecOpMasking { pub fn is_enabled (& self) -> bool { match self { VecOpMasking :: Enabled { .. } => true , VecOpMasking :: Disabled => false , } } pub fn encode (& self) -> u32 { match self { VecOpMasking :: Enabled { .. } => 0 , VecOpMasking :: Disabled => 1 , } } }
};
}
