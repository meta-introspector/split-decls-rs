macro_rules! deps {
    () => {
        UnindexedFieldsAttr!();
        RawAttribute!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl RawAttribute for UnindexedFieldsAttr { fn key (& self) -> & str { "unindexed_fields" } }
    };
}

impl_24!()