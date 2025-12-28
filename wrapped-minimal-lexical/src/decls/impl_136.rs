macro_rules! deps {
    () => {
        StackVec!();
        Limb!();
    };
}

macro_rules! impl_136 {
    () => {
        deps!();
        impl ops :: Deref for StackVec { type Target = [bigint :: Limb] ; # [inline] fn deref (& self) -> & [bigint :: Limb] { unsafe { let ptr = self . data . as_ptr () as * const bigint :: Limb ; slice :: from_raw_parts (ptr , self . len ()) } } }
    };
}

impl_136!()