macro_rules! deps {
    () => {
        LazyBuffer!();
    };
}

macro_rules! CombinationsWithReplacementGeneric {
    () => {
        deps!();
        # [doc = " An iterator to iterate through all the `n`-length combinations in an iterator, with replacement."] # [doc = ""] # [doc = " See [`.combinations_with_replacement()`](crate::Itertools::combinations_with_replacement)"] # [doc = " for more information."] # [derive (Clone)] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] pub struct CombinationsWithReplacementGeneric < I , Idx > where I : Iterator , I :: Item : Clone , { indices : Idx , pool : LazyBuffer < I > , first : bool , }
    };
}

CombinationsWithReplacementGeneric!()