macro_rules! deps {
    () => {
        VersionFileId!();
    };
}

macro_rules! impl_1147 {
    () => {
        deps!();
        impl IdPrivate for VersionFileId { fn new (id : usize) -> Self { VersionFileId (id) } }
    };
}

impl_1147!();