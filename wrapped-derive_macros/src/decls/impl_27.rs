macro_rules! deps {
    () => {
        UnlabeledVariantsAttr!();
        RawAttribute!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl RawAttribute for UnlabeledVariantsAttr { fn key (& self) -> & str { "unlabeled_variants" } }
    };
}

impl_27!()