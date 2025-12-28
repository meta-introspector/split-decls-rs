macro_rules! deps {
    () => {
        CombinationsGeneric!();
    };
}

macro_rules! ArrayCombinations {
    () => {
        deps!();
        # [doc = " Iterator for const generic combinations returned by [`.array_combinations()`](crate::Itertools::array_combinations)"] pub type ArrayCombinations < I , const K : usize > = CombinationsGeneric < I , [usize ; K] > ;
    };
}

ArrayCombinations!()