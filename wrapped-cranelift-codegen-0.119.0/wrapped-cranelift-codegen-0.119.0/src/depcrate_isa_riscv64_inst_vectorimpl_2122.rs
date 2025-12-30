// Generated macro for impl_2122 (impl)
macro_rules! Depcrate_isa_riscv64_inst_vectorimpl_2122 {
() => {
// Module: crate::isa::riscv64::inst::vector
// Provides: {"impl_2122"}
// Dependencies: {}
impl VecLmul { pub fn encode (& self) -> u32 { match self { VecLmul :: LmulF8 => 0b101 , VecLmul :: LmulF4 => 0b110 , VecLmul :: LmulF2 => 0b111 , VecLmul :: Lmul1 => 0b000 , VecLmul :: Lmul2 => 0b001 , VecLmul :: Lmul4 => 0b010 , VecLmul :: Lmul8 => 0b011 , } } }
};
}
