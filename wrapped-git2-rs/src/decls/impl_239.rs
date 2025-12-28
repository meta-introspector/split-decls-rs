macro_rules! deps {
    () => {
        Buf!();
    };
}

macro_rules! impl_239 {
    () => {
        deps!();
        impl Deref for Buf { type Target = [u8] ; fn deref (& self) -> & [u8] { unsafe { slice :: from_raw_parts (self . raw . ptr as * const u8 , self . raw . size as usize) } } }
    };
}

impl_239!()