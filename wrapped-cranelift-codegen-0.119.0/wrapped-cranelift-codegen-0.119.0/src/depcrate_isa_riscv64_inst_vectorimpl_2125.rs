// Generated macro for impl_2125 (impl)
macro_rules! Depcrate_isa_riscv64_inst_vectorimpl_2125 {
() => {
// Module: crate::isa::riscv64::inst::vector
// Provides: {"impl_2125"}
// Dependencies: {}
impl fmt :: Display for VecTailMode { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match self { VecTailMode :: Agnostic => write ! (f , "ta") , VecTailMode :: Undisturbed => write ! (f , "tu") , } } }
};
}
