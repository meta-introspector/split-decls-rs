macro_rules! deps {
    () => {
        Data!();
    };
}

macro_rules! impl_86 {
    () => {
        deps!();
        impl Deref for Data { type Target = [u8] ; fn deref (& self) -> & [u8] { if self . ptr . is_null () { & [] } else { unsafe { core :: slice :: from_raw_parts (self . ptr , self . len) } } } }
    };
}

impl_86!();