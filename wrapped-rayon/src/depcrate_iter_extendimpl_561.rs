// Generated macro for impl_561 (impl)
macro_rules! Depcrate_iter_extendimpl_561 {
() => {
// Module: crate::iter::extend
// Provides: {"impl_561"}
// Dependencies: {}
# [doc = " Extends a string with strings from a parallel iterator."] impl ParallelExtend < String > for String { fn par_extend < I > (& mut self , par_iter : I) where I : IntoParallelIterator < Item = String > , { extend_reserved ! (self , par_iter , string_len) ; } }
};
}
