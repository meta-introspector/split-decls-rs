macro_rules! deps {
    () => {
        Limb!();
        HeapVec!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        impl ops :: Deref for HeapVec { type Target = [bigint :: Limb] ; # [inline] fn deref (& self) -> & [bigint :: Limb] { & self . data } }
    };
}

impl_66!()