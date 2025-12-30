// Generated macro for Fold (struct)
macro_rules! Depcrate_iter_foldFold {
() => {
// Module: crate::iter::fold
// Provides: {"Fold"}
// Dependencies: {}
# [doc = " `Fold` is an iterator that applies a function over an iterator producing a single value."] # [doc = " This struct is created by the [`fold()`] method on [`ParallelIterator`]"] # [doc = ""] # [doc = " [`fold()`]: ParallelIterator::fold()"] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] # [derive (Clone)] pub struct Fold < I , ID , F > { base : I , identity : ID , fold_op : F , }
};
}
