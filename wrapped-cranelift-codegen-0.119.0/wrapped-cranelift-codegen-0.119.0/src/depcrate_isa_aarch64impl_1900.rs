// Generated macro for impl_1900 (impl)
macro_rules! Depcrate_isa_aarch64impl_1900 {
() => {
// Module: crate::isa::aarch64
// Provides: {"impl_1900"}
// Dependencies: {}
impl AArch64Backend { # [doc = " Create a new AArch64 backend with the given (shared) flags."] pub fn new_with_flags (triple : Triple , flags : shared_settings :: Flags , isa_flags : aarch64_settings :: Flags ,) -> AArch64Backend { AArch64Backend { triple , flags , isa_flags , } } # [doc = " This performs lowering to VCode, register-allocates the code, computes block layout and"] # [doc = " finalizes branches. The result is ready for binary emission."] fn compile_vcode (& self , func : & Function , domtree : & DominatorTree , ctrl_plane : & mut ControlPlane ,) -> CodegenResult < (VCode < inst :: Inst > , regalloc2 :: Output) > { let emit_info = EmitInfo :: new (self . flags . clone ()) ; let sigs = SigSet :: new :: < abi :: AArch64MachineDeps > (func , & self . flags) ? ; let abi = abi :: AArch64Callee :: new (func , self , & self . isa_flags , & sigs) ? ; compile :: compile :: < AArch64Backend > (func , domtree , self , abi , emit_info , sigs , ctrl_plane) } }
};
}
