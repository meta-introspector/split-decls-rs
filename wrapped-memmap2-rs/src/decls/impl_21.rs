macro_rules! deps {
    () => {
        Mmap!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl Deref for Mmap { type Target = [u8] ; # [inline] fn deref (& self) -> & [u8] { unsafe { slice :: from_raw_parts (self . inner . ptr () , self . inner . len ()) } } }
    };
}

impl_21!();