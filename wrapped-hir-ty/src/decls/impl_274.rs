macro_rules! deps {
    () => {
        Diverges!();
    };
}

macro_rules! impl_274 {
    () => {
        deps!();
        impl std :: ops :: BitOr for Diverges { type Output = Self ; fn bitor (self , other : Self) -> Self { std :: cmp :: max (self , other) } }
    };
}

impl_274!();