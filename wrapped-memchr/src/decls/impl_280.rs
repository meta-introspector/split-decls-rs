macro_rules! deps {
    () => {
        CowBytes!();
    };
}

macro_rules! impl_280 {
    () => {
        deps!();
        impl < 'a > ops :: Deref for CowBytes < 'a > { type Target = [u8] ; # [inline (always)] fn deref (& self) -> & [u8] { self . as_slice () } }
    };
}

impl_280!();