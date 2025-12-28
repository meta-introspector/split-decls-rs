macro_rules! deps {
    () => {
        ConsTuples!();
        ConsTuplesFn!();
    };
}

macro_rules! cons_tuples {
    () => {
        deps!();
        # [doc = " Create an iterator that maps for example iterators of"] # [doc = " `((A, B), C)` to `(A, B, C)`."] pub fn cons_tuples < I > (iterable : I) -> ConsTuples < I :: IntoIter > where I : IntoIterator , { ConsTuples { iter : iterable . into_iter () , f : ConsTuplesFn , } }
    };
}

cons_tuples!();