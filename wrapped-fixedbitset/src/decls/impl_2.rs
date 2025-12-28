macro_rules! deps {
    () => {
        Block!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl Block { # [inline] pub const fn is_empty (self) -> bool { self . 0 == Self :: NONE . 0 } # [inline] pub fn andnot (self , other : Self) -> Self { Self (! other . 0 & self . 0) } }
    };
}

impl_2!()