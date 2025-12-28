macro_rules! deps {
    () => {
        ParallelIterator!();
    };
}

macro_rules! Fold {
    () => {
        deps!();
        # [doc = " `Fold` is an iterator that applies a function over an iterator producing a single value."] # [doc = " This struct is created by the [`fold()`] method on [`ParallelIterator`]"] # [doc = ""] # [doc = " [`fold()`]: ParallelIterator::fold()"] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] # [derive (Clone)] pub struct Fold < I , ID , F > { base : I , identity : ID , fold_op : F , }
    };
}

Fold!();