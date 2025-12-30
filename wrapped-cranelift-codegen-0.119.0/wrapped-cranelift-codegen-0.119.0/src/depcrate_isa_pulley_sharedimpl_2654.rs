// Generated macro for impl_2654 (impl)
macro_rules! Depcrate_isa_pulley_sharedimpl_2654 {
() => {
// Module: crate::isa::pulley_shared
// Provides: {"impl_2654"}
// Dependencies: {}
impl < P > PulleyBackend < P > where P : PulleyTargetKind , { # [doc = " Create a new pulley backend with the given (shared) flags."] pub fn new_with_flags (triple : Triple , flags : shared_settings :: Flags , isa_flags : PulleyFlags ,) -> Self { PulleyBackend { pulley_target : PhantomData , triple , flags , isa_flags , } } # [doc = " This performs lowering to VCode, register-allocates the code, computes block layout and"] # [doc = " finalizes branches. The result is ready for binary emission."] fn compile_vcode (& self , func : & ir :: Function , domtree : & DominatorTree , ctrl_plane : & mut ControlPlane ,) -> CodegenResult < (VCode < inst :: InstAndKind < P > > , regalloc2 :: Output) > { let emit_info = EmitInfo :: new (func . signature . call_conv , self . flags . clone () , self . isa_flags . clone () ,) ; let sigs = SigSet :: new :: < abi :: PulleyMachineDeps < P > > (func , & self . flags) ? ; let abi = abi :: PulleyCallee :: new (func , self , & self . isa_flags , & sigs) ? ; machinst :: compile :: < Self > (func , domtree , self , abi , emit_info , sigs , ctrl_plane) } }
};
}
