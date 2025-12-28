macro_rules! deps {
    () => {
        HeapVec!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        impl cmp :: Ord for HeapVec { # [inline] fn cmp (& self , other : & Self) -> cmp :: Ordering { bigint :: compare (self , other) } }
    };
}

impl_65!();