macro_rules! deps {
    () => {
        ConsTuplesFn!();
        MapSpecialCase!();
    };
}

macro_rules! ConsTuples {
    () => {
        deps!();
        # [doc = " An iterator that maps an iterator of tuples like"] # [doc = " `((A, B), C)` to an iterator of `(A, B, C)`."] # [doc = ""] # [doc = " Used by the `iproduct!()` macro."] pub type ConsTuples < I > = MapSpecialCase < I , ConsTuplesFn > ;
    };
}

ConsTuples!();