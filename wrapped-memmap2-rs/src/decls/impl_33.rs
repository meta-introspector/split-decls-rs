macro_rules! deps {
    () => {
        MmapMut!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl DerefMut for MmapMut { # [inline] fn deref_mut (& mut self) -> & mut [u8] { unsafe { slice :: from_raw_parts_mut (self . inner . mut_ptr () , self . inner . len ()) } } }
    };
}

impl_33!()