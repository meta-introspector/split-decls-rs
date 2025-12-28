macro_rules! deps {
    () => {
        RangedI64ValueParser!();
    };
}

macro_rules! impl_293 {
    () => {
        deps!();
        impl < T : TryFrom < i64 > + Clone + Send + Sync > Default for RangedI64ValueParser < T > { fn default () -> Self { Self :: new () } }
    };
}

impl_293!()