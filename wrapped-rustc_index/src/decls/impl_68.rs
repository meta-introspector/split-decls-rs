macro_rules! deps {
    () => {
        FiniteBitSetTy!();
        FiniteBitSet!();
    };
}

macro_rules! impl_68 {
    () => {
        deps!();
        impl < T : FiniteBitSetTy > Default for FiniteBitSet < T > { fn default () -> Self { Self :: new_empty () } }
    };
}

impl_68!()