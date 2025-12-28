macro_rules! deps {
    () => {
        Id!();
        VersionFileId!();
    };
}

macro_rules! impl_1146 {
    () => {
        deps!();
        impl Id for VersionFileId { fn index (& self) -> usize { self . 0 } }
    };
}

impl_1146!();