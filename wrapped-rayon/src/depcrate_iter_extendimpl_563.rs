// Generated macro for impl_563 (impl)
macro_rules! Depcrate_iter_extendimpl_563 {
() => {
// Module: crate::iter::extend
// Provides: {"impl_563"}
// Dependencies: {}
# [doc = " Extends a string with string slices from a parallel iterator."] impl < 'a > ParallelExtend < Cow < 'a , str > > for String { fn par_extend < I > (& mut self , par_iter : I) where I : IntoParallelIterator < Item = Cow < 'a , str > > , { extend_reserved ! (self , par_iter , string_len) ; } }
};
}
