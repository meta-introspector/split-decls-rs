macro_rules! deps {
    () => {
        CanonicalDecomposition!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        # [doc = " ✨ *Enabled with the `harfbuzz_traits` Cargo feature.*"] impl DecomposeFunc for & '_ CanonicalDecomposition { fn decompose (& self , ab : char) -> Option < (char , char) > { DecomposeFunc :: decompose (& self . as_borrowed () , ab) } }
    };
}

impl_9!();