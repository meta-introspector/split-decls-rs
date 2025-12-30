// Generated macro for impl_568 (impl)
macro_rules! Depcrate_iter_extendimpl_568 {
() => {
// Module: crate::iter::extend
// Provides: {"impl_568"}
// Dependencies: {}
# [doc = " Collapses all unit items from a parallel iterator into one."] impl ParallelExtend < () > for () { fn par_extend < I > (& mut self , par_iter : I) where I : IntoParallelIterator < Item = () > , { par_iter . into_par_iter () . drive_unindexed (NoopConsumer) } }
};
}
