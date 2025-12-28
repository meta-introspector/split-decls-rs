macro_rules! deps {
    () => {
        CanonicalComposition!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        # [doc = " ✨ *Enabled with the `harfbuzz_traits` Cargo feature.*"] impl ComposeFunc for CanonicalComposition { fn compose (& self , a : char , b : char) -> Option < char > { ComposeFunc :: compose (& self . as_borrowed () , a , b) } }
    };
}

impl_5!();