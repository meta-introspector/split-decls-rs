macro_rules! deps {
    () => {
        VersionFileId!();
        Id!();
        VersionFile!();
        Item!();
    };
}

macro_rules! impl_1149 {
    () => {
        deps!();
        impl < 'data > Item for VersionFile < 'data > { type Id = VersionFileId ; fn is_deleted (& self) -> bool { self . delete } }
    };
}

impl_1149!();