macro_rules! deps {
    () => {
        PotentialCodePoint!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl From < PotentialCodePoint > for u32 { fn from (x : PotentialCodePoint) -> Self { let [a0 , a1 , a2] = x . 0 ; u32 :: from_le_bytes ([a0 , a1 , a2 , 0]) } }
    };
}

impl_12!();