// Generated macro for impl_2481 (impl)
macro_rules! Depcrate_isa_s390ximpl_2481 {
() => {
// Module: crate::isa::s390x
// Provides: {"impl_2481"}
// Dependencies: {}
impl S390xBackend { # [doc = " Create a new IBM Z backend with the given (shared) flags."] pub fn new_with_flags (triple : Triple , flags : shared_settings :: Flags , isa_flags : s390x_settings :: Flags ,) -> S390xBackend { S390xBackend { triple , flags , isa_flags , } } # [doc = " This performs lowering to VCode, register-allocates the code, computes block layout and"] # [doc = " finalizes branches. The result is ready for binary emission."] fn compile_vcode (& self , func : & Function , domtree : & DominatorTree , ctrl_plane : & mut ControlPlane ,) -> CodegenResult < (VCode < inst :: Inst > , regalloc2 :: Output) > { let emit_info = EmitInfo :: new (self . isa_flags . clone ()) ; let sigs = SigSet :: new :: < abi :: S390xMachineDeps > (func , & self . flags) ? ; let abi = abi :: S390xCallee :: new (func , self , & self . isa_flags , & sigs) ? ; compile :: compile :: < S390xBackend > (func , domtree , self , abi , emit_info , sigs , ctrl_plane) } }
};
}
