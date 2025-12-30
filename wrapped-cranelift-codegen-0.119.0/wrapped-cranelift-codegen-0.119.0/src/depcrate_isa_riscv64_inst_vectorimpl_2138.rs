// Generated macro for impl_2138 (impl)
macro_rules! Depcrate_isa_riscv64_inst_vectorimpl_2138 {
() => {
// Module: crate::isa::riscv64::inst::vector
// Provides: {"impl_2138"}
// Dependencies: {}
impl VecInstOverlapInfo for VecAluOpRRRR { fn forbids_src_dst_overlaps (& self) -> bool { match self { VecAluOpRRRR :: Vslide1upVX => true , _ => false , } } }
};
}
