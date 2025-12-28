macro_rules! deps {
    () => {
        CanonicalComposition!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        # [doc = " ✨ *Enabled with the `harfbuzz_traits` Cargo feature.*"] impl ComposeFunc for & '_ CanonicalComposition { fn compose (& self , a : char , b : char) -> Option < char > { ComposeFunc :: compose (& self . as_borrowed () , a , b) } }
    };
}

impl_6!();