macro_rules! deps {
    () => {
        OsRng!();
    };
}

macro_rules! impl_269 {
    () => {
        deps!();
        impl TryCryptoRng for OsRng { }
    };
}

impl_269!()