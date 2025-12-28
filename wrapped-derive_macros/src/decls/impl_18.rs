macro_rules! deps {
    () => {
        RawAttribute!();
        SkipAttr!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl RawAttribute for SkipAttr { fn key (& self) -> & str { "skip" } }
    };
}

impl_18!()