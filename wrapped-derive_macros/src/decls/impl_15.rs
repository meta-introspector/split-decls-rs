macro_rules! deps {
    () => {
        IndexAttr!();
        RawAttribute!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl RawAttribute for IndexAttr { fn key (& self) -> & str { "index" } }
    };
}

impl_15!();