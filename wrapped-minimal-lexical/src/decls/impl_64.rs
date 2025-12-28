macro_rules! deps {
    () => {
        HeapVec!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        impl cmp :: PartialOrd for HeapVec { # [inline] fn partial_cmp (& self , other : & Self) -> Option < cmp :: Ordering > { Some (bigint :: compare (self , other)) } }
    };
}

impl_64!();