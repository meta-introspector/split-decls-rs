// Generated macro for s390x (module)
macro_rules! Depcrate_imp_atomic128s390x {
() => {
// Module: crate::imp::atomic128
// Provides: {"s390x"}
// Dependencies: {}
# [cfg (all (target_arch = "s390x" , not (all (any (miri , portable_atomic_sanitize_thread) , not (portable_atomic_atomic_intrinsics))) , not (portable_atomic_no_asm) ,))] # [cfg_attr (any (miri , portable_atomic_sanitize_thread) , path = "intrinsics.rs")] pub (super) mod s390x ;
};
}
