// Generated macro for impl_562 (impl)
macro_rules! Depcrate_iter_extendimpl_562 {
() => {
// Module: crate::iter::extend
// Provides: {"impl_562"}
// Dependencies: {}
# [doc = " Extends a string with boxed strings from a parallel iterator."] impl ParallelExtend < Box < str > > for String { fn par_extend < I > (& mut self , par_iter : I) where I : IntoParallelIterator < Item = Box < str > > , { extend_reserved ! (self , par_iter , string_len) ; } }
};
}
