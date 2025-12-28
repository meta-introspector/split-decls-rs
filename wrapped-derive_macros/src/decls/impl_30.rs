macro_rules! deps {
    () => {
        RawAttribute!();
        UnindexedVariantsAttr!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl RawAttribute for UnindexedVariantsAttr { fn key (& self) -> & str { "unindexed_variants" } }
    };
}

impl_30!()