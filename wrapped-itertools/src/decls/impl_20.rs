macro_rules! deps {
    () => {
        CountItem!();
        NoCount!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl < T > CountItem < T > for NoCount { type CItem = T ; # [inline (always)] fn new (t : T) -> T { t } }
    };
}

impl_20!();