macro_rules! deps {
    () => {
        RawAttribute!();
        LabelAttr!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl RawAttribute for LabelAttr { fn key (& self) -> & str { "label" } }
    };
}

impl_11!()