macro_rules! deps {
    () => {
        DYNAMIC_TIME_ZONE_INFORMATION!();
    };
}

macro_rules! impl_553 {
    () => {
        deps!();
        impl Default for DYNAMIC_TIME_ZONE_INFORMATION { fn default () -> Self { unsafe { core :: mem :: zeroed () } } }
    };
}

impl_553!();