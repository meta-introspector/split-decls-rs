macro_rules! deps {
    () => {
        PotentialCodePoint!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        # [doc = " This impl requires enabling the optional `zerovec` Cargo feature"] # [cfg (feature = "zerovec")] unsafe impl zerovec :: ule :: EqULE for PotentialCodePoint { }
    };
}

impl_4!()