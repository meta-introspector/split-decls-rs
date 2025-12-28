macro_rules! deps {
    () => {
        Buf!();
    };
}

macro_rules! impl_240 {
    () => {
        deps!();
        impl DerefMut for Buf { fn deref_mut (& mut self) -> & mut [u8] { unsafe { slice :: from_raw_parts_mut (self . raw . ptr as * mut u8 , self . raw . size as usize) } } }
    };
}

impl_240!()