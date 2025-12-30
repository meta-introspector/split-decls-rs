// Generated macro for stable_sort (function)
macro_rules! Depcrate_slicestable_sort {
() => {
// Module: crate::slice
// Provides: {"stable_sort"}
// Dependencies: {}
# [inline] # [cfg (not (no_global_oom_handling))] fn stable_sort < T , F > (v : & mut [T] , mut is_less : F) where F : FnMut (& T , & T) -> bool , { sort :: stable :: sort :: < T , F , Vec < T > > (v , & mut is_less) ; }
};
}
