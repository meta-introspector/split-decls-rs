macro_rules! deps {
    () => {
        PotentialCodePoint!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        # [doc = " This impl requires enabling the optional `zerovec` Cargo feature"] # [cfg (feature = "zerovec")] impl zerovec :: ule :: AsULE for PotentialCodePoint { type ULE = zerovec :: ule :: RawBytesULE < 3 > ; # [inline] fn to_unaligned (self) -> Self :: ULE { zerovec :: ule :: RawBytesULE (self . 0) } # [inline] fn from_unaligned (unaligned : Self :: ULE) -> Self { Self (unaligned . 0) } }
    };
}

impl_3!();