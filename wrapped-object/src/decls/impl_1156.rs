macro_rules! deps {
    () => {
        Id!();
        VersionId!();
    };
}

macro_rules! impl_1156 {
    () => {
        deps!();
        impl Id for VersionId { fn index (& self) -> usize { self . 0 - VERSION_ID_BASE } }
    };
}

impl_1156!();