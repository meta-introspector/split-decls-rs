macro_rules! deps {
    () => {
        MmapMut!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl Deref for MmapMut { type Target = [u8] ; # [inline] fn deref (& self) -> & [u8] { unsafe { slice :: from_raw_parts (self . inner . ptr () , self . inner . len ()) } } }
    };
}

impl_27!()