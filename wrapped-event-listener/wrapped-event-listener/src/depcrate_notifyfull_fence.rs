// Generated macro for full_fence (function)
macro_rules! Depcrate_notifyfull_fence {
() => {
// Module: crate::notify
// Provides: {"full_fence"}
// Dependencies: {}
# [doc = " Equivalent to `atomic::fence(Ordering::SeqCst)`, but in some cases faster."] # [inline] pub (super) fn full_fence () { # [cfg (all (any (target_arch = "x86" , target_arch = "x86_64") , not (miri) , not (loom)))] { use core :: { arch :: asm , cell :: UnsafeCell } ; let a = UnsafeCell :: new (0_usize) ; unsafe { # [cfg (target_pointer_width = "64")] asm ! ("lock not qword ptr [{0}]" , in (reg) a . get () , options (nostack , preserves_flags)) ; # [cfg (target_pointer_width = "32")] asm ! ("lock not dword ptr [{0:e}]" , in (reg) a . get () , options (nostack , preserves_flags)) ; } return ; } # [allow (unreachable_code)] { atomic :: fence (Ordering :: SeqCst) ; } }
};
}
