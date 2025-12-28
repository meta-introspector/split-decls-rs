macro_rules! deps {
    () => {
        Version!();
        Id!();
        Item!();
        VersionId!();
    };
}

macro_rules! impl_1160 {
    () => {
        deps!();
        impl < 'data > Item for Version < 'data > { type Id = VersionId ; fn is_deleted (& self) -> bool { self . delete } }
    };
}

impl_1160!()