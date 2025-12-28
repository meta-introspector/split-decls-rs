macro_rules! deps {
    () => {
        LazyBuffer!();
    };
}

macro_rules! CombinationsGeneric {
    () => {
        deps!();
        # [doc = " An iterator to iterate through all the `k`-length combinations in an iterator."] # [doc = ""] # [doc = " See [`.combinations()`](crate::Itertools::combinations) and [`.array_combinations()`](crate::Itertools::array_combinations) for more information."] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] pub struct CombinationsGeneric < I : Iterator , Idx > { indices : Idx , pool : LazyBuffer < I > , first : bool , }
    };
}

CombinationsGeneric!()