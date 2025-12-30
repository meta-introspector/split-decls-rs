// Generated macro for impl_2151 (impl)
macro_rules! Depcrate_isa_riscv64_inst_vectorimpl_2151 {
() => {
// Module: crate::isa::riscv64::inst::vector
// Provides: {"impl_2151"}
// Dependencies: {}
impl fmt :: Display for VecAluOpRR { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . write_str (match self { VecAluOpRR :: VmvSX => "vmv.s.x" , VecAluOpRR :: VmvXS => "vmv.x.s" , VecAluOpRR :: VfmvSF => "vfmv.s.f" , VecAluOpRR :: VfmvFS => "vfmv.f.s" , VecAluOpRR :: VfsqrtV => "vfsqrt.v" , VecAluOpRR :: VzextVF2 => "vzext.vf2" , VecAluOpRR :: VzextVF4 => "vzext.vf4" , VecAluOpRR :: VzextVF8 => "vzext.vf8" , VecAluOpRR :: VsextVF2 => "vsext.vf2" , VecAluOpRR :: VsextVF4 => "vsext.vf4" , VecAluOpRR :: VsextVF8 => "vsext.vf8" , VecAluOpRR :: VmvVV => "vmv.v.v" , VecAluOpRR :: VmvVX => "vmv.v.x" , VecAluOpRR :: VfmvVF => "vfmv.v.f" , VecAluOpRR :: VfcvtxufV => "vfcvt.xu.f.v" , VecAluOpRR :: VfcvtxfV => "vfcvt.x.f.v" , VecAluOpRR :: VfcvtrtzxufV => "vfcvt.rtz.xu.f.v" , VecAluOpRR :: VfcvtrtzxfV => "vfcvt.rtz.x.f.v" , VecAluOpRR :: VfcvtfxuV => "vfcvt.f.xu.v" , VecAluOpRR :: VfcvtfxV => "vfcvt.f.x.v" , VecAluOpRR :: VfwcvtffV => "vfwcvt.f.f.v" , VecAluOpRR :: VfncvtffW => "vfncvt.f.f.w" , }) } }
};
}
