// Generated macro for impl_2250 (impl)
macro_rules! Depcrate_isa_riscv64impl_2250 {
() => {
// Module: crate::isa::riscv64
// Provides: {"impl_2250"}
// Dependencies: {}
impl Riscv64Backend { # [doc = " Create a new riscv64 backend with the given (shared) flags."] pub fn new_with_flags (triple : Triple , flags : shared_settings :: Flags , isa_flags : riscv_settings :: Flags ,) -> Riscv64Backend { Riscv64Backend { triple , flags , isa_flags , } } # [doc = " This performs lowering to VCode, register-allocates the code, computes block layout and"] # [doc = " finalizes branches. The result is ready for binary emission."] fn compile_vcode (& self , func : & Function , domtree : & DominatorTree , ctrl_plane : & mut ControlPlane ,) -> CodegenResult < (VCode < inst :: Inst > , regalloc2 :: Output) > { let emit_info = EmitInfo :: new (self . flags . clone () , self . isa_flags . clone ()) ; let sigs = SigSet :: new :: < abi :: Riscv64MachineDeps > (func , & self . flags) ? ; let abi = abi :: Riscv64Callee :: new (func , self , & self . isa_flags , & sigs) ? ; compile :: compile :: < Riscv64Backend > (func , domtree , self , abi , emit_info , sigs , ctrl_plane) } }
};
}
