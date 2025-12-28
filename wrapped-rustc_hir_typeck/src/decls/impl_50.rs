macro_rules! deps {
    () => {
        Diverges!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl ops :: BitAnd for Diverges { type Output = Self ; fn bitand (self , other : Self) -> Self { cmp :: min (self , other) } }
    };
}

impl_50!();