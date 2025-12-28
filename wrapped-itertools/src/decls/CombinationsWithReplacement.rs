macro_rules! deps {
    () => {
        CombinationsWithReplacementGeneric!();
    };
}

macro_rules! CombinationsWithReplacement {
    () => {
        deps!();
        # [doc = " Iterator for `Box<[I]>` valued combinations_with_replacement returned by [`.combinations_with_replacement()`](crate::Itertools::combinations_with_replacement)"] pub type CombinationsWithReplacement < I > = CombinationsWithReplacementGeneric < I , Box < [usize] > > ;
    };
}

CombinationsWithReplacement!()