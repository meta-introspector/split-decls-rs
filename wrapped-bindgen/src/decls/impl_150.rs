macro_rules! deps {
    () => {
        ThereIsNoIteratorInRepetition!();
    };
}

macro_rules! impl_150 {
    () => {
        deps!();
        impl BitOr < Self > for ThereIsNoIteratorInRepetition { type Output = Self ; fn bitor (self , _rhs : Self) -> Self { Self } }
    };
}

impl_150!()