// Generated macro for collect_extended (function)
macro_rules! Depcrate_iter_from_par_itercollect_extended {
() => {
// Module: crate::iter::from_par_iter
// Provides: {"collect_extended"}
// Dependencies: {}
# [doc = " Creates an empty default collection and extends it."] fn collect_extended < C , I > (par_iter : I) -> C where I : IntoParallelIterator , C : ParallelExtend < I :: Item > + Default , { let mut collection = C :: default () ; collection . par_extend (par_iter) ; collection }
};
}
