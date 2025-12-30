// Generated macro for with_feat_dit (function)
macro_rules! Depcrate_ditwith_feat_dit {
() => {
// Module: crate::dit
// Provides: {"with_feat_dit"}
// Dependencies: {}
# [doc = " Wraps code with DIT when `FEAT_DIT` was detected but not `FEAT_SB`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `FEAT_DIT` must have been detected."] # [inline] # [target_feature (enable = "dit")] unsafe fn with_feat_dit < T , F > (f : F) -> T where F : FnOnce () -> T , { struct Guard { dit : u64 , } impl Drop for Guard { # [inline] fn drop (& mut self) { unsafe { wsr64_dit (self . dit) } ; } } let _guard = Guard { dit : unsafe { rsr64_dit () } , } ; unsafe { enable_dit () } ; synchronization_barrier () ; f () }
};
}
