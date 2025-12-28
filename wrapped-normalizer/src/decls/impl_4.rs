macro_rules! deps {
    () => {
        CanonicalCompositionBorrowed!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        # [doc = " ✨ *Enabled with the `harfbuzz_traits` Cargo feature.*"] impl ComposeFunc for CanonicalCompositionBorrowed < '_ > { fn compose (& self , a : char , b : char) -> Option < char > { CanonicalCompositionBorrowed :: compose (* self , a , b) } }
    };
}

impl_4!();