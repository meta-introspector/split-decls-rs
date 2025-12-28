macro_rules! hwcaps {
    () => {
        # [doc = " Linux hardware capabilities mapped to target features."] # [doc = ""] # [doc = " Note that LLVM target features are coarser grained than what Linux supports"] # [doc = " and imply more capabilities under each feature. This module attempts to"] # [doc = " provide that mapping accordingly."] # [cfg (target_os = "linux")] pub mod hwcaps { use libc :: c_ulong ; pub const UAL : c_ulong = libc :: HWCAP_LOONGARCH_UAL ; pub const FPU : c_ulong = libc :: HWCAP_LOONGARCH_FPU ; pub const LSX : c_ulong = libc :: HWCAP_LOONGARCH_LSX ; pub const LASX : c_ulong = libc :: HWCAP_LOONGARCH_LASX ; pub const LVZ : c_ulong = libc :: HWCAP_LOONGARCH_LVZ ; pub const LBT_X86 : c_ulong = libc :: HWCAP_LOONGARCH_LBT_X86 ; pub const LBT_ARM : c_ulong = libc :: HWCAP_LOONGARCH_LBT_ARM ; pub const LBT_MIPS : c_ulong = libc :: HWCAP_LOONGARCH_LBT_MIPS ; }
    };
}

hwcaps!();