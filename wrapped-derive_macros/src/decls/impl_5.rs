macro_rules! deps {
    () => {
        TagAttr!();
        RawAttribute!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl RawAttribute for TagAttr { fn key (& self) -> & str { "tag" } }
    };
}

impl_5!();