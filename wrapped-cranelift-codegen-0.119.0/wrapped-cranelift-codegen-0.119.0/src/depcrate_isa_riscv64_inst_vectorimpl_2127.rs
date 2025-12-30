// Generated macro for impl_2127 (impl)
macro_rules! Depcrate_isa_riscv64_inst_vectorimpl_2127 {
() => {
// Module: crate::isa::riscv64::inst::vector
// Provides: {"impl_2127"}
// Dependencies: {}
impl fmt :: Display for VecMaskMode { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match self { VecMaskMode :: Agnostic => write ! (f , "ma") , VecMaskMode :: Undisturbed => write ! (f , "mu") , } } }
};
}
