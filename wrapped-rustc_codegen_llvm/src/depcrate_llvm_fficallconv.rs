// Generated macro for CallConv (enum)
macro_rules! Depcrate_llvm_ffiCallConv {
() => {
// Module: crate::llvm::ffi
// Provides: {"CallConv"}
// Dependencies: {}
# [doc = " LLVM CallingConv::ID. Should we wrap this?"] # [doc = ""] # [doc = " See <https://github.com/llvm/llvm-project/blob/main/llvm/include/llvm/IR/CallingConv.h>"] # [derive (Copy , Clone , PartialEq , Debug , TryFromU32)] # [repr (C)] pub (crate) enum CallConv { CCallConv = 0 , FastCallConv = 8 , ColdCallConv = 9 , PreserveMost = 14 , PreserveAll = 15 , Tail = 18 , X86StdcallCallConv = 64 , X86FastcallCallConv = 65 , ArmAapcsCallConv = 67 , Msp430Intr = 69 , X86_ThisCall = 70 , PtxKernel = 71 , X86_64_SysV = 78 , X86_64_Win64 = 79 , X86_VectorCall = 80 , X86_Intr = 83 , AvrNonBlockingInterrupt = 84 , AvrInterrupt = 85 , AmdgpuKernel = 91 , }
};
}
