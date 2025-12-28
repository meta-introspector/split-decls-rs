macro_rules! deps {
    () => {
        CanonicalCombiningClassMap!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        # [doc = " ✨ *Enabled with the `harfbuzz_traits` Cargo feature.*"] impl CombiningClassFunc for & '_ CanonicalCombiningClassMap { fn combining_class (& self , ch : char) -> u8 { CombiningClassFunc :: combining_class (& self . as_borrowed () , ch) } }
    };
}

impl_12!();