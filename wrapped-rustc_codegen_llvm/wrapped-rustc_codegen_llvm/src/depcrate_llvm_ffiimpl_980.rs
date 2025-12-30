// Generated macro for impl_980 (impl)
macro_rules! Depcrate_llvm_ffiimpl_980 {
() => {
// Module: crate::llvm::ffi
// Provides: {"impl_980"}
// Dependencies: {}
impl IntPredicate { pub (crate) fn from_generic (intpre : rustc_codegen_ssa :: common :: IntPredicate) -> Self { use rustc_codegen_ssa :: common :: IntPredicate as Common ; match intpre { Common :: IntEQ => Self :: IntEQ , Common :: IntNE => Self :: IntNE , Common :: IntUGT => Self :: IntUGT , Common :: IntUGE => Self :: IntUGE , Common :: IntULT => Self :: IntULT , Common :: IntULE => Self :: IntULE , Common :: IntSGT => Self :: IntSGT , Common :: IntSGE => Self :: IntSGE , Common :: IntSLT => Self :: IntSLT , Common :: IntSLE => Self :: IntSLE , } } }
};
}
