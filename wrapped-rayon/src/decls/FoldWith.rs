macro_rules! deps {
    () => {
        ParallelIterator!();
    };
}

macro_rules! FoldWith {
    () => {
        deps!();
        # [doc = " `FoldWith` is an iterator that applies a function over an iterator producing a single value."] # [doc = " This struct is created by the [`fold_with()`] method on [`ParallelIterator`]"] # [doc = ""] # [doc = " [`fold_with()`]: ParallelIterator::fold_with()"] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] # [derive (Clone)] pub struct FoldWith < I , U , F > { base : I , item : U , fold_op : F , }
    };
}

FoldWith!();