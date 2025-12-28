macro_rules! deps {
    () => {
        VersionId!();
        Version!();
    };
}

macro_rules! impl_1161 {
    () => {
        deps!();
        impl < 'data > Version < 'data > { # [doc = " The ID used for referring to this version."] pub fn id (& self) -> VersionId { self . id } }
    };
}

impl_1161!()