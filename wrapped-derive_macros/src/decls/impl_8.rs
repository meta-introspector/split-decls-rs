macro_rules! deps {
    () => {
        RawAttribute!();
        DataTagAttr!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl RawAttribute for DataTagAttr { fn key (& self) -> & str { "data_tag" } }
    };
}

impl_8!()