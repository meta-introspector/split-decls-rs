macro_rules! deps {
    () => {
        Diverges!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl ops :: BitOr for Diverges { type Output = Self ; fn bitor (self , other : Self) -> Self { cmp :: max (self , other) } }
    };
}

impl_51!()