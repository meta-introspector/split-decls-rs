macro_rules! deps {
    () => {
        RawAttribute!();
        FlattenAttr!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl RawAttribute for FlattenAttr { fn key (& self) -> & str { "flatten" } }
    };
}

impl_39!();