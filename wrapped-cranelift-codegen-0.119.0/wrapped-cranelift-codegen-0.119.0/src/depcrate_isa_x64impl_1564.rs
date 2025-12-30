// Generated macro for impl_1564 (impl)
macro_rules! Depcrate_isa_x64impl_1564 {
() => {
// Module: crate::isa::x64
// Provides: {"impl_1564"}
// Dependencies: {}
impl X64Backend { # [doc = " Create a new X64 backend with the given (shared) flags."] fn new_with_flags (triple : Triple , flags : Flags , x64_flags : x64_settings :: Flags) -> Self { Self { triple , flags , x64_flags , } } fn compile_vcode (& self , func : & Function , domtree : & DominatorTree , ctrl_plane : & mut ControlPlane ,) -> CodegenResult < (VCode < inst :: Inst > , regalloc2 :: Output) > { let emit_info = EmitInfo :: new (self . flags . clone () , self . x64_flags . clone ()) ; let sigs = SigSet :: new :: < abi :: X64ABIMachineSpec > (func , & self . flags) ? ; let abi = abi :: X64Callee :: new (func , self , & self . x64_flags , & sigs) ? ; compile :: compile :: < Self > (func , domtree , self , abi , emit_info , sigs , ctrl_plane) } }
};
}
