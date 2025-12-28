macro_rules! deps {
    () => {
        PotentialCodePoint!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl Ord for PotentialCodePoint { fn cmp (& self , other : & Self) -> Ordering { let a = u32 :: from (* self) ; let b = u32 :: from (* other) ; a . cmp (& b) } }
    };
}

impl_11!()