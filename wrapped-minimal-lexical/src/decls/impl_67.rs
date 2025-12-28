macro_rules! deps {
    () => {
        HeapVec!();
        Limb!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl ops :: DerefMut for HeapVec { # [inline] fn deref_mut (& mut self) -> & mut [bigint :: Limb] { & mut self . data } }
    };
}

impl_67!();