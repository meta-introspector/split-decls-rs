// Generated macro for impl_560 (impl)
macro_rules! Depcrate_iter_extendimpl_560 {
() => {
// Module: crate::iter::extend
// Provides: {"impl_560"}
// Dependencies: {}
# [doc = " Extends a string with string slices from a parallel iterator."] impl < 'a > ParallelExtend < & 'a str > for String { fn par_extend < I > (& mut self , par_iter : I) where I : IntoParallelIterator < Item = & 'a str > , { extend_reserved ! (self , par_iter , string_len) ; } }
};
}
