// Generated macro for fast_collect (function)
macro_rules! Depcrate_iter_extendfast_collect {
() => {
// Module: crate::iter::extend
// Provides: {"fast_collect"}
// Dependencies: {}
pub (super) fn fast_collect < I , T > (pi : I) -> Either < Vec < T > , LinkedList < Vec < T > > > where I : IntoParallelIterator < Item = T > , T : Send , { let par_iter = pi . into_par_iter () ; match par_iter . opt_len () { Some (len) => { let mut vec = Vec :: new () ; super :: collect :: special_extend (par_iter , len , & mut vec) ; Either :: Left (vec) } None => Either :: Right (par_iter . drive_unindexed (ListVecConsumer)) , } }
};
}
