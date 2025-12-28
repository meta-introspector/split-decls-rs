macro_rules! deps {
    () => {
        RawAttribute!();
        DynamicAttr!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl RawAttribute for DynamicAttr { fn key (& self) -> & str { "dynamic" } }
    };
}

impl_33!();