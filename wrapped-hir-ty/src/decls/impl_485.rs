macro_rules! deps {
    () => {
        DisplayTarget!();
        HirDatabase!();
    };
}

macro_rules! impl_485 {
    () => {
        deps!();
        impl DisplayTarget { pub fn from_crate (db : & dyn HirDatabase , krate : Crate) -> Self { let edition = krate . data (db) . edition ; Self { krate , edition } } }
    };
}

impl_485!();