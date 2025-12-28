macro_rules! deps {
    () => {
        Block!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl Block { # [inline] pub fn is_empty (self) -> bool { ! v128_any_true (self . 0) } # [inline] pub fn andnot (self , other : Self) -> Self { Self (v128_andnot (self . 0 , other . 0)) } }
    };
}

impl_45!()