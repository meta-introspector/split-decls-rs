macro_rules! deps {
    () => {
        Section!();
        Id!();
        Item!();
        SectionId!();
    };
}

macro_rules! impl_1123 {
    () => {
        deps!();
        impl < 'data > Item for Section < 'data > { type Id = SectionId ; fn is_deleted (& self) -> bool { self . delete } }
    };
}

impl_1123!()