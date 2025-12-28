macro_rules! deps {
    () => {
        Limb!();
        StackVec!();
    };
}

macro_rules! impl_137 {
    () => {
        deps!();
        impl ops :: DerefMut for StackVec { # [inline] fn deref_mut (& mut self) -> & mut [bigint :: Limb] { unsafe { let ptr = self . data . as_mut_ptr () as * mut bigint :: Limb ; slice :: from_raw_parts_mut (ptr , self . len ()) } } }
    };
}

impl_137!()