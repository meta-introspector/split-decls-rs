macro_rules! deps {
    () => {
        RawAttribute!();
        UnindexedFieldsAttr!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl RawAttribute for UnindexedFieldsAttr { fn key (& self) -> & str { "unindexed_fields" } }
    };
}

impl_24!();