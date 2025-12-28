macro_rules! deps {
    () => {
        CombinationsWithReplacementGeneric!();
    };
}

macro_rules! ArrayCombinationsWithReplacement {
    () => {
        deps!();
        # [doc = " Iterator for const generic combinations_with_replacement returned by [`.array_combinations_with_replacement()`](crate::Itertools::array_combinations_with_replacement)"] pub type ArrayCombinationsWithReplacement < I , const K : usize > = CombinationsWithReplacementGeneric < I , [usize ; K] > ;
    };
}

ArrayCombinationsWithReplacement!()