macro_rules! deps {
    () => {
        IndexStr!();
    };
}

macro_rules! impl_327 {
    () => {
        deps!();
        impl < 'a > Into < & 'a [u8] > for IndexStr < 'a > { fn into (self) -> & 'a [u8] { self . string } }
    };
}

impl_327!()