macro_rules! deps {
    () => {
        VersionFileId!();
        VersionFile!();
    };
}

macro_rules! impl_1150 {
    () => {
        deps!();
        impl < 'data > VersionFile < 'data > { # [doc = " The ID used for referring to this filename."] pub fn id (& self) -> VersionFileId { self . id } }
    };
}

impl_1150!()