macro_rules! deps {
    () => {
        StringOrVec!();
    };
}

macro_rules! impl_179 {
    () => {
        deps!();
        impl StringOrVec { pub fn iter < 'a > (& 'a self) -> std :: slice :: Iter < 'a , String > { self . 0 . iter () } }
    };
}

impl_179!();