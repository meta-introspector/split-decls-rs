// Generated macro for zero_extend64_ptr (function)
macro_rules! Depcrate_utilszero_extend64_ptr {
() => {
// Module: crate::utils
// Provides: {"zero_extend64_ptr"}
// Dependencies: {}
# [doc = " Zero-extends the given 32-bit pointer to `MaybeUninit<u64>`."] # [doc = " This is used for 64-bit architecture's 32-bit ABI (e.g., AArch64 ILP32 ABI)."] # [doc = " See ptr_reg! macro in src/gen/utils.rs for details."] # [cfg (not (portable_atomic_no_asm_maybe_uninit))] # [cfg (target_pointer_width = "32")] # [allow (dead_code)] # [inline] pub (crate) const fn zero_extend64_ptr (v : * mut ()) -> core :: mem :: MaybeUninit < u64 > { # [repr (C)] struct ZeroExtended { # [cfg (target_endian = "big")] pad : * mut () , v : * mut () , # [cfg (target_endian = "little")] pad : * mut () , } unsafe { core :: mem :: transmute (ZeroExtended { v , pad : core :: ptr :: null_mut () }) } }
};
}
