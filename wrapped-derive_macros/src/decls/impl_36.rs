macro_rules! deps {
    () => {
        RawAttribute!();
        TransparentAttr!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl RawAttribute for TransparentAttr { fn key (& self) -> & str { "transparent" } }
    };
}

impl_36!();