// Generated macro for impl_2132 (impl)
macro_rules! Depcrate_isa_riscv64_inst_vectorimpl_2132 {
() => {
// Module: crate::isa::riscv64::inst::vector
// Provides: {"impl_2132"}
// Dependencies: {}
impl VState { pub fn from_type (ty : Type) -> Self { VState { avl : VecAvl :: _static (ty . lane_count ()) , vtype : VType { sew : VecElementWidth :: from_type (ty) , lmul : VecLmul :: Lmul1 , tail_mode : VecTailMode :: Agnostic , mask_mode : VecMaskMode :: Agnostic , } , } } }
};
}
