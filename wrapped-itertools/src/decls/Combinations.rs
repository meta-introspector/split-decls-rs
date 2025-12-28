macro_rules! deps {
    () => {
        CombinationsGeneric!();
    };
}

macro_rules! Combinations {
    () => {
        deps!();
        # [doc = " Iterator for `Vec` valued combinations returned by [`.combinations()`](crate::Itertools::combinations)"] pub type Combinations < I > = CombinationsGeneric < I , Vec < usize > > ;
    };
}

Combinations!()