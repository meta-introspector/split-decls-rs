// Generated macro for create_sub_word_mask_values (function)
macro_rules! Depcrate_utilscreate_sub_word_mask_values {
() => {
// Module: crate::utils
// Provides: {"create_sub_word_mask_values"}
// Dependencies: {}
# [cfg (any (target_arch = "riscv32" , target_arch = "riscv64"))] # [allow (dead_code)] # [inline] pub (crate) fn create_sub_word_mask_values < T > (ptr : * mut T) -> (* mut MinWord , RetInt , RetInt) { # [cfg (portable_atomic_no_strict_provenance)] use self :: ptr :: PtrExt as _ ; use core :: mem ; const SHIFT_MASK : bool = ! cfg ! (any (target_arch = "bpf" , target_arch = "loongarch32" , target_arch = "loongarch64" , target_arch = "mips" , target_arch = "mips32r6" , target_arch = "mips64" , target_arch = "mips64r6" , target_arch = "riscv32" , target_arch = "riscv64" , target_arch = "s390x" , target_arch = "sparc" , target_arch = "sparc64" , target_arch = "xtensa" ,)) ; let ptr_mask = mem :: size_of :: < MinWord > () - 1 ; let aligned_ptr = ptr . with_addr (ptr . addr () & ! ptr_mask) as * mut MinWord ; let ptr_lsb = if SHIFT_MASK { ptr . addr () & ptr_mask } else { ptr . addr () } ; let shift = if cfg ! (any (target_endian = "little" , target_arch = "s390x")) { ptr_lsb . wrapping_mul (8) } else { (ptr_lsb ^ (mem :: size_of :: < MinWord > () - mem :: size_of :: < T > ())) . wrapping_mul (8) } ; let mut mask : RetInt = (1 << (mem :: size_of :: < T > () * 8)) - 1 ; if SHIFT_MASK { mask <<= shift ; } # [allow (clippy :: cast_possible_truncation)] { (aligned_ptr , shift as RetInt , mask) } }
};
}
