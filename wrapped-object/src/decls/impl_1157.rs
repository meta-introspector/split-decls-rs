macro_rules! deps {
    () => {
        VersionId!();
    };
}

macro_rules! impl_1157 {
    () => {
        deps!();
        impl IdPrivate for VersionId { fn new (id : usize) -> Self { VersionId (VERSION_ID_BASE + id) } }
    };
}

impl_1157!()