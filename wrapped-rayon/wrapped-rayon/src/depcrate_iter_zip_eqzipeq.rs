// Generated macro for ZipEq (struct)
macro_rules! Depcrate_iter_zip_eqZipEq {
() => {
// Module: crate::iter::zip_eq
// Provides: {"ZipEq"}
// Dependencies: {}
# [doc = " An [`IndexedParallelIterator`] that iterates over two parallel iterators of equal"] # [doc = " length simultaneously."] # [doc = ""] # [doc = " This struct is created by the [`zip_eq`] method on [`IndexedParallelIterator`],"] # [doc = " see its documentation for more information."] # [doc = ""] # [doc = " [`zip_eq`]: IndexedParallelIterator::zip_eq()"] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] # [derive (Debug , Clone)] pub struct ZipEq < A , B > { zip : Zip < A , B > , }
};
}
