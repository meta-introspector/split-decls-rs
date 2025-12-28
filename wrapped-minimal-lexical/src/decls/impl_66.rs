macro_rules! deps {
    () => {
        HeapVec!();
        Limb!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        impl ops :: Deref for HeapVec { type Target = [bigint :: Limb] ; # [inline] fn deref (& self) -> & [bigint :: Limb] { & self . data } }
    };
}

impl_66!();