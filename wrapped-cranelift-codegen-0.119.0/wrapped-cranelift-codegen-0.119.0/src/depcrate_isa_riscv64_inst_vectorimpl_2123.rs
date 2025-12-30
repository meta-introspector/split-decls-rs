// Generated macro for impl_2123 (impl)
macro_rules! Depcrate_isa_riscv64_inst_vectorimpl_2123 {
() => {
// Module: crate::isa::riscv64::inst::vector
// Provides: {"impl_2123"}
// Dependencies: {}
impl fmt :: Display for VecLmul { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match self { VecLmul :: LmulF8 => write ! (f , "mf8") , VecLmul :: LmulF4 => write ! (f , "mf4") , VecLmul :: LmulF2 => write ! (f , "mf2") , VecLmul :: Lmul1 => write ! (f , "m1") , VecLmul :: Lmul2 => write ! (f , "m2") , VecLmul :: Lmul4 => write ! (f , "m4") , VecLmul :: Lmul8 => write ! (f , "m8") , } } }
};
}
