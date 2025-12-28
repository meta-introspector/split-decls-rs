macro_rules! deps {
    () => {
        IndexStr!();
    };
}

macro_rules! impl_325 {
    () => {
        deps!();
        impl < 'a > AsRef < [u8] > for IndexStr < 'a > { # [inline] fn as_ref (& self) -> & [u8] { self . string } }
    };
}

impl_325!();