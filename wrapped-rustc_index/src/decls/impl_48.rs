macro_rules! deps {
    () => {
        Idx!();
        GrowableBitSet!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl < T : Idx > Default for GrowableBitSet < T > { fn default () -> Self { GrowableBitSet :: new_empty () } }
    };
}

impl_48!();