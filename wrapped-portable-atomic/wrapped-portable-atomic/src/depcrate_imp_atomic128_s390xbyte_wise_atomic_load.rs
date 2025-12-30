// Generated macro for byte_wise_atomic_load (function)
macro_rules! Depcrate_imp_atomic128_s390xbyte_wise_atomic_load {
() => {
// Module: crate::imp::atomic128::s390x
// Provides: {"byte_wise_atomic_load"}
// Dependencies: {}
# [cfg (not (any (target_feature = "load-store-on-cond" , portable_atomic_target_feature = "load-store-on-cond" ,)))] # [inline] unsafe fn byte_wise_atomic_load (src : * const u128) -> u128 { unsafe { let (out_hi , out_lo) ; asm ! ("lg {out_hi}, 8({src})" , "lg {out_lo}, 0({src})" , src = in (reg) src , out_hi = out (reg) out_hi , out_lo = out (reg) out_lo , options (pure , nostack , preserves_flags , readonly) ,) ; U128 { pair : Pair { hi : out_hi , lo : out_lo } } . whole } }
};
}
