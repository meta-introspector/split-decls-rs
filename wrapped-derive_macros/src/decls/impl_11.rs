macro_rules! deps {
    () => {
        LabelAttr!();
        RawAttribute!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl RawAttribute for LabelAttr { fn key (& self) -> & str { "label" } }
    };
}

impl_11!();