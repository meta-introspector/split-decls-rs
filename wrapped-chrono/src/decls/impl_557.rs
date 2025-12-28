macro_rules! deps {
    () => {
        TIME_ZONE_INFORMATION!();
    };
}

macro_rules! impl_557 {
    () => {
        deps!();
        impl Default for TIME_ZONE_INFORMATION { fn default () -> Self { unsafe { core :: mem :: zeroed () } } }
    };
}

impl_557!()