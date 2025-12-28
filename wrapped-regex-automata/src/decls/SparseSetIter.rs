macro_rules! deps {
    () => {
        StateID!();
    };
}

macro_rules! SparseSetIter {
    () => {
        deps!();
        # [doc = " An iterator over all elements in a sparse set."] # [doc = ""] # [doc = " The lifetime `'a` refers to the lifetime of the set being iterated over."] # [derive (Debug)] pub (crate) struct SparseSetIter < 'a > (core :: slice :: Iter < 'a , StateID >) ;
    };
}

SparseSetIter!();