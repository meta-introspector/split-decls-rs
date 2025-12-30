// Generated macro for conv_to_fn_attribute (function)
macro_rules! Depcrate_abiconv_to_fn_attribute {
() => {
// Module: crate::abi
// Provides: {"conv_to_fn_attribute"}
// Dependencies: {}
# [cfg (feature = "master")] pub fn conv_to_fn_attribute < 'gcc > (conv : CanonAbi , arch : & str) -> Option < FnAttribute < 'gcc > > { let attribute = match conv { CanonAbi :: C | CanonAbi :: Rust => return None , CanonAbi :: RustCold => FnAttribute :: Cold , CanonAbi :: Custom => return None , CanonAbi :: Arm (arm_call) => match arm_call { ArmCall :: CCmseNonSecureCall => FnAttribute :: ArmCmseNonsecureCall , ArmCall :: CCmseNonSecureEntry => FnAttribute :: ArmCmseNonsecureEntry , ArmCall :: Aapcs => FnAttribute :: ArmPcs ("aapcs") , } , CanonAbi :: GpuKernel => { if arch == "amdgpu" { FnAttribute :: GcnAmdGpuHsaKernel } else if arch == "nvptx64" { FnAttribute :: NvptxKernel } else { panic ! ("Architecture {} does not support GpuKernel calling convention" , arch) ; } } CanonAbi :: Interrupt (interrupt_kind) => match interrupt_kind { InterruptKind :: Avr => FnAttribute :: AvrSignal , InterruptKind :: AvrNonBlocking => FnAttribute :: AvrInterrupt , InterruptKind :: Msp430 => FnAttribute :: Msp430Interrupt , InterruptKind :: RiscvMachine => FnAttribute :: RiscvInterrupt ("machine") , InterruptKind :: RiscvSupervisor => FnAttribute :: RiscvInterrupt ("supervisor") , InterruptKind :: X86 => FnAttribute :: X86Interrupt , } , CanonAbi :: X86 (x86_call) => match x86_call { X86Call :: Fastcall => FnAttribute :: X86FastCall , X86Call :: Stdcall => FnAttribute :: X86Stdcall , X86Call :: Thiscall => FnAttribute :: X86ThisCall , X86Call :: Vectorcall => return None , X86Call :: SysV64 => FnAttribute :: X86SysvAbi , X86Call :: Win64 => FnAttribute :: X86MsAbi , } , } ; Some (attribute) }
};
}
