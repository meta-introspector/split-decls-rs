// Generated macro for impl_2147 (impl)
macro_rules! Depcrate_isa_riscv64_inst_vectorimpl_2147 {
() => {
// Module: crate::isa::riscv64::inst::vector
// Provides: {"impl_2147"}
// Dependencies: {}
impl VecInstOverlapInfo for VecAluOpRRImm5 { fn forbids_src_dst_overlaps (& self) -> bool { match self { VecAluOpRRImm5 :: VrgatherVI => true , _ => false , } } fn forbids_mask_dst_overlaps (& self) -> bool { match self { VecAluOpRRImm5 :: VmseqVI | VecAluOpRRImm5 :: VmsneVI | VecAluOpRRImm5 :: VmsleuVI | VecAluOpRRImm5 :: VmsleVI | VecAluOpRRImm5 :: VmsgtuVI | VecAluOpRRImm5 :: VmsgtVI => false , _ => true , } } }
};
}
