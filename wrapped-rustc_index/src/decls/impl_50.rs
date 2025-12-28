macro_rules! deps {
    () => {
        DenseBitSet!();
        Idx!();
        GrowableBitSet!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl < T : Idx > From < DenseBitSet < T > > for GrowableBitSet < T > { fn from (bit_set : DenseBitSet < T >) -> Self { Self { bit_set } } }
    };
}

impl_50!();