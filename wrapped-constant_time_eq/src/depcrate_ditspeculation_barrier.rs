// Generated macro for speculation_barrier (function)
macro_rules! Depcrate_ditspeculation_barrier {
() => {
// Module: crate::dit
// Provides: {"speculation_barrier"}
// Dependencies: {}
# [doc = " Equivalent to `__asm__ __volatile__(\"sb\" ::: \"memory\")`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Must be called only when `FEAT_SB` is implemented."] # [inline] # [target_feature (enable = "sb")] unsafe fn speculation_barrier () { unsafe { asm ! ("sb" , options (nostack)) ; } }
};
}
