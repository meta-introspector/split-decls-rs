// Generated macro for impl_2150 (impl)
macro_rules! Depcrate_isa_riscv64_inst_vectorimpl_2150 {
() => {
// Module: crate::isa::riscv64::inst::vector
// Provides: {"impl_2150"}
// Dependencies: {}
impl VecInstOverlapInfo for VecAluOpRR { fn forbids_src_dst_overlaps (& self) -> bool { match self { VecAluOpRR :: VzextVF2 | VecAluOpRR :: VzextVF4 | VecAluOpRR :: VzextVF8 | VecAluOpRR :: VsextVF2 | VecAluOpRR :: VsextVF4 | VecAluOpRR :: VsextVF8 | VecAluOpRR :: VfwcvtffV | VecAluOpRR :: VfncvtffW => true , _ => false , } } }
};
}
