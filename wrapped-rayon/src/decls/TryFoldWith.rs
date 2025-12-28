macro_rules! deps {
    () => {
        ParallelIterator!();
    };
}

macro_rules! TryFoldWith {
    () => {
        deps!();
        # [doc = " `TryFoldWith` is an iterator that applies a function over an iterator producing a single value."] # [doc = " This struct is created by the [`try_fold_with()`] method on [`ParallelIterator`]"] # [doc = ""] # [doc = " [`try_fold_with()`]: ParallelIterator::try_fold_with()"] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] # [derive (Clone)] pub struct TryFoldWith < I , U : Try , F > { base : I , item : U :: Output , fold_op : F , }
    };
}

TryFoldWith!()