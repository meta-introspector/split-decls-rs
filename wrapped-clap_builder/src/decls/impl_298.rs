macro_rules! deps {
    () => {
        RangedU64ValueParser!();
    };
}

macro_rules! impl_298 {
    () => {
        deps!();
        impl < T : TryFrom < u64 > > Default for RangedU64ValueParser < T > { fn default () -> Self { Self :: new () } }
    };
}

impl_298!()