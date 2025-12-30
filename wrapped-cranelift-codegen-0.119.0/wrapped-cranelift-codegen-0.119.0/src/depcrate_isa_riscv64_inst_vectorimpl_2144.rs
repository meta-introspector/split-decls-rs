// Generated macro for impl_2144 (impl)
macro_rules! Depcrate_isa_riscv64_inst_vectorimpl_2144 {
() => {
// Module: crate::isa::riscv64::inst::vector
// Provides: {"impl_2144"}
// Dependencies: {}
impl VecInstOverlapInfo for VecAluOpRRR { fn forbids_src_dst_overlaps (& self) -> bool { match self { VecAluOpRRR :: VrgatherVV | VecAluOpRRR :: VrgatherVX | VecAluOpRRR :: VcompressVM | VecAluOpRRR :: VwadduVV | VecAluOpRRR :: VwadduVX | VecAluOpRRR :: VwaddVV | VecAluOpRRR :: VwaddVX | VecAluOpRRR :: VwadduWV | VecAluOpRRR :: VwadduWX | VecAluOpRRR :: VwaddWV | VecAluOpRRR :: VwaddWX | VecAluOpRRR :: VwsubuVV | VecAluOpRRR :: VwsubuVX | VecAluOpRRR :: VwsubVV | VecAluOpRRR :: VwsubVX | VecAluOpRRR :: VwsubuWV | VecAluOpRRR :: VwsubuWX | VecAluOpRRR :: VwsubWV | VecAluOpRRR :: VwsubWX => true , _ => false , } } fn forbids_mask_dst_overlaps (& self) -> bool { match self { VecAluOpRRR :: VredmaxuVS | VecAluOpRRR :: VredminuVS | VecAluOpRRR :: VmandMM | VecAluOpRRR :: VmorMM | VecAluOpRRR :: VmnandMM | VecAluOpRRR :: VmnorMM | VecAluOpRRR :: VmseqVX | VecAluOpRRR :: VmsneVX | VecAluOpRRR :: VmsltuVX | VecAluOpRRR :: VmsltVX | VecAluOpRRR :: VmsleuVX | VecAluOpRRR :: VmsleVX | VecAluOpRRR :: VmsgtuVX | VecAluOpRRR :: VmsgtVX | VecAluOpRRR :: VmfeqVV | VecAluOpRRR :: VmfneVV | VecAluOpRRR :: VmfltVV | VecAluOpRRR :: VmfleVV | VecAluOpRRR :: VmfeqVF | VecAluOpRRR :: VmfneVF | VecAluOpRRR :: VmfltVF | VecAluOpRRR :: VmfleVF | VecAluOpRRR :: VmfgtVF | VecAluOpRRR :: VmfgeVF => false , _ => true , } } }
};
}
