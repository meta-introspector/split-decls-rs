// Generated macro for hwcaps (module)
macro_rules! Depcrate_aarch64hwcaps {
() => {
// Module: crate::aarch64
// Provides: {"hwcaps"}
// Dependencies: {}
# [doc = " Linux hardware capabilities mapped to target features."] # [doc = ""] # [doc = " Note that LLVM target features are coarser grained than what Linux supports"] # [doc = " and imply more capabilities under each feature. This module attempts to"] # [doc = " provide that mapping accordingly."] # [doc = ""] # [doc = " See this issue for more info: <https://github.com/RustCrypto/utils/issues/395>"] # [cfg (any (target_os = "linux" , target_os = "android"))] pub mod hwcaps { use libc :: c_ulong ; pub const AES : c_ulong = libc :: HWCAP_AES | libc :: HWCAP_PMULL ; pub const DIT : c_ulong = libc :: HWCAP_DIT ; pub const SHA2 : c_ulong = libc :: HWCAP_SHA2 ; pub const SHA3 : c_ulong = libc :: HWCAP_SHA3 | libc :: HWCAP_SHA512 ; pub const SM4 : c_ulong = libc :: HWCAP_SM3 | libc :: HWCAP_SM4 ; }
};
}
