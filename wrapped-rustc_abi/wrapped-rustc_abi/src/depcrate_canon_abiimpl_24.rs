// Generated macro for impl_24 (impl)
macro_rules! Depcrate_canon_abiimpl_24 {
() => {
// Module: crate::canon_abi
// Provides: {"impl_24"}
// Dependencies: {}
impl fmt :: Display for CanonAbi { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let erased_abi = match self { CanonAbi :: C => ExternAbi :: C { unwind : false } , CanonAbi :: Rust => ExternAbi :: Rust , CanonAbi :: RustCold => ExternAbi :: RustCold , CanonAbi :: Custom => ExternAbi :: Custom , CanonAbi :: Arm (arm_call) => match arm_call { ArmCall :: Aapcs => ExternAbi :: Aapcs { unwind : false } , ArmCall :: CCmseNonSecureCall => ExternAbi :: CmseNonSecureCall , ArmCall :: CCmseNonSecureEntry => ExternAbi :: CmseNonSecureEntry , } , CanonAbi :: GpuKernel => ExternAbi :: GpuKernel , CanonAbi :: Interrupt (interrupt_kind) => match interrupt_kind { InterruptKind :: Avr => ExternAbi :: AvrInterrupt , InterruptKind :: AvrNonBlocking => ExternAbi :: AvrNonBlockingInterrupt , InterruptKind :: Msp430 => ExternAbi :: Msp430Interrupt , InterruptKind :: RiscvMachine => ExternAbi :: RiscvInterruptM , InterruptKind :: RiscvSupervisor => ExternAbi :: RiscvInterruptS , InterruptKind :: X86 => ExternAbi :: X86Interrupt , } , CanonAbi :: X86 (x86_call) => match x86_call { X86Call :: Fastcall => ExternAbi :: Fastcall { unwind : false } , X86Call :: Stdcall => ExternAbi :: Stdcall { unwind : false } , X86Call :: SysV64 => ExternAbi :: SysV64 { unwind : false } , X86Call :: Thiscall => ExternAbi :: Thiscall { unwind : false } , X86Call :: Vectorcall => ExternAbi :: Vectorcall { unwind : false } , X86Call :: Win64 => ExternAbi :: Win64 { unwind : false } , } , } ; erased_abi . as_str () . fmt (f) } }
};
}
