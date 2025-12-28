macro_rules! deps {
    () => {
        Data!();
    };
}

macro_rules! impl_87 {
    () => {
        deps!();
        impl core :: ops :: DerefMut for Data { fn deref_mut (& mut self) -> & mut [u8] { if self . ptr . is_null () { & mut [] } else { unsafe { core :: slice :: from_raw_parts_mut (self . ptr , self . len) } } } }
    };
}

impl_87!()