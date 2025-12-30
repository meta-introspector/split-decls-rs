// Generated macro for impl_986 (impl)
macro_rules! Depcrate_llvm_ffiimpl_986 {
() => {
// Module: crate::llvm::ffi
// Provides: {"impl_986"}
// Dependencies: {}
impl AtomicRmwBinOp { pub (crate) fn from_generic (op : rustc_codegen_ssa :: common :: AtomicRmwBinOp) -> Self { use rustc_codegen_ssa :: common :: AtomicRmwBinOp as Common ; match op { Common :: AtomicXchg => Self :: AtomicXchg , Common :: AtomicAdd => Self :: AtomicAdd , Common :: AtomicSub => Self :: AtomicSub , Common :: AtomicAnd => Self :: AtomicAnd , Common :: AtomicNand => Self :: AtomicNand , Common :: AtomicOr => Self :: AtomicOr , Common :: AtomicXor => Self :: AtomicXor , Common :: AtomicMax => Self :: AtomicMax , Common :: AtomicMin => Self :: AtomicMin , Common :: AtomicUMax => Self :: AtomicUMax , Common :: AtomicUMin => Self :: AtomicUMin , } } }
};
}
