// Generated macro for TryFold (struct)
macro_rules! Depcrate_iter_try_foldTryFold {
() => {
// Module: crate::iter::try_fold
// Provides: {"TryFold"}
// Dependencies: {}
# [doc = " `TryFold` is an iterator that applies a function over an iterator producing a single value."] # [doc = " This struct is created by the [`try_fold()`] method on [`ParallelIterator`]"] # [doc = ""] # [doc = " [`try_fold()`]: ParallelIterator::try_fold()"] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] # [derive (Clone)] pub struct TryFold < I , U , ID , F > { base : I , identity : ID , fold_op : F , marker : PhantomData < U > , }
};
}
