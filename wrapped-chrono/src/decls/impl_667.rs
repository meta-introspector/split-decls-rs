macro_rules! deps {
    () => {
        Utc!();
        Offset!();
        FixedOffset!();
    };
}

macro_rules! impl_667 {
    () => {
        deps!();
        impl Offset for Utc { fn fix (& self) -> FixedOffset { FixedOffset :: east_opt (0) . unwrap () } }
    };
}

impl_667!()