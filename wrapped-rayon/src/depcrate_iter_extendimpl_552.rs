// Generated macro for impl_552 (impl)
macro_rules! Depcrate_iter_extendimpl_552 {
() => {
// Module: crate::iter::extend
// Provides: {"impl_552"}
// Dependencies: {}
# [doc = " Extends an OS-string with string slices from a parallel iterator."] impl < 'a > ParallelExtend < Cow < 'a , OsStr > > for OsString { fn par_extend < I > (& mut self , par_iter : I) where I : IntoParallelIterator < Item = Cow < 'a , OsStr > > , { extend_reserved ! (self , par_iter , osstring_len) ; } }
};
}
