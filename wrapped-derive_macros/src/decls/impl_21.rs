macro_rules! deps {
    () => {
        UnlabeledFieldsAttr!();
        RawAttribute!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl RawAttribute for UnlabeledFieldsAttr { fn key (& self) -> & str { "unlabeled_fields" } }
    };
}

impl_21!();