macro_rules! deps {
    () => {
        Idx!();
        DenseBitSet!();
        GrowableBitSet!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl < T : Idx > From < GrowableBitSet < T > > for DenseBitSet < T > { fn from (bit_set : GrowableBitSet < T >) -> Self { bit_set . bit_set } }
    };
}

impl_15!();