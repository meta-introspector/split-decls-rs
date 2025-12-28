macro_rules! deps {
    () => {
        CanonicalCombiningClassMapBorrowed!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        # [doc = " ✨ *Enabled with the `harfbuzz_traits` Cargo feature.*"] impl CombiningClassFunc for CanonicalCombiningClassMapBorrowed < '_ > { fn combining_class (& self , ch : char) -> u8 { self . get_u8 (ch) } }
    };
}

impl_10!();