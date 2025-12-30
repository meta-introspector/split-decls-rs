// Generated macro for with_dit (function)
macro_rules! Depcrate_ditwith_dit {
() => {
// Module: crate::dit
// Provides: {"with_dit"}
// Dependencies: {}
# [doc = " Runs code with the hardware DIT feature enabled when possible."] # [inline] pub (crate) fn with_dit < T , F > (f : F) -> T where F : FnOnce () -> T , { match get_aarch64_dit_sb_features () { Features :: DitSb => { unsafe { with_feat_dit_sb (f) } } Features :: DitOnly => { unsafe { with_feat_dit (f) } } Features :: Neither => f () , } }
};
}
