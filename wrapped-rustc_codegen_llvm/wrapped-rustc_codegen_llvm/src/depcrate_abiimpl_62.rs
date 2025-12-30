// Generated macro for impl_62 (impl)
macro_rules! Depcrate_abiimpl_62 {
() => {
// Module: crate::abi
// Provides: {"impl_62"}
// Dependencies: {}
impl llvm :: CallConv { pub (crate) fn from_conv (conv : CanonAbi , arch : & str) -> Self { match conv { CanonAbi :: C | CanonAbi :: Rust => llvm :: CCallConv , CanonAbi :: RustCold => llvm :: PreserveMost , CanonAbi :: Custom => llvm :: CCallConv , CanonAbi :: GpuKernel => { if arch == "amdgpu" { llvm :: AmdgpuKernel } else if arch == "nvptx64" { llvm :: PtxKernel } else { panic ! ("Architecture {arch} does not support GpuKernel calling convention") ; } } CanonAbi :: Interrupt (interrupt_kind) => match interrupt_kind { InterruptKind :: Avr => llvm :: AvrInterrupt , InterruptKind :: AvrNonBlocking => llvm :: AvrNonBlockingInterrupt , InterruptKind :: Msp430 => llvm :: Msp430Intr , InterruptKind :: RiscvMachine | InterruptKind :: RiscvSupervisor => llvm :: CCallConv , InterruptKind :: X86 => llvm :: X86_Intr , } , CanonAbi :: Arm (arm_call) => match arm_call { ArmCall :: Aapcs => llvm :: ArmAapcsCallConv , ArmCall :: CCmseNonSecureCall | ArmCall :: CCmseNonSecureEntry => llvm :: CCallConv , } , CanonAbi :: X86 (x86_call) => match x86_call { X86Call :: Fastcall => llvm :: X86FastcallCallConv , X86Call :: Stdcall => llvm :: X86StdcallCallConv , X86Call :: SysV64 => llvm :: X86_64_SysV , X86Call :: Thiscall => llvm :: X86_ThisCall , X86Call :: Vectorcall => llvm :: X86_VectorCall , X86Call :: Win64 => llvm :: X86_64_Win64 , } , } } }
};
}
