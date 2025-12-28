macro_rules! deps {
    () => {
        Diverges!();
    };
}

macro_rules! impl_87 {
    () => {
        deps!();
        impl std :: ops :: BitAnd for Diverges { type Output = Self ; fn bitand (self , other : Self) -> Self { std :: cmp :: min (self , other) } }
    };
}

impl_87!();