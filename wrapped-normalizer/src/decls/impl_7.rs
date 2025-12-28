macro_rules! deps {
    () => {
        CanonicalDecompositionBorrowed!();
        Decomposed!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        # [doc = " ✨ *Enabled with the `harfbuzz_traits` Cargo feature.*"] impl DecomposeFunc for CanonicalDecompositionBorrowed < '_ > { fn decompose (& self , ab : char) -> Option < (char , char) > { match CanonicalDecompositionBorrowed :: decompose (self , ab) { Decomposed :: Default => None , Decomposed :: Expansion (first , second) => Some ((first , second)) , Decomposed :: Singleton (single) => Some ((single , '\0')) , } } }
    };
}

impl_7!()